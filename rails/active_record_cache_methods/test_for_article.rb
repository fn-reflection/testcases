# frozen_string_literal: true

# ruby test_for_article.rb で実行可能
# rubyは3.1以降が必要です(ハッシュの省略記法使ってるので)
# bundlerも必要です
require 'bundler/inline'

# 必要なgemをインストール
gemfile(true) do
  source 'https://rubygems.org'
  gem 'rails'
  gem 'sqlite3', '~> 1.4'
end

require 'active_record'
require 'minitest/autorun'
require 'logger'

ActiveRecord::Base.establish_connection(adapter: 'sqlite3', database: ':memory:')

# データスキーマ定義
ActiveRecord::Schema.define do
  # ユーザデータ
  create_table :users do |t|
    t.text :name
  end

  # ユーザが投稿したテキスト
  create_table :posts do |t|
    t.integer :user_id
    t.text :title
  end

  # ユーザが投稿したテキストに対するコメント
  create_table :post_comments do |t|
    t.integer :post_id
    t.text :comment
  end

  create_table :post_comment_reviews do |t|
    t.integer :post_comment_id
    t.integer :grade
  end
end

# モデル定義

# ユーザ
class User < ActiveRecord::Base
  has_many :posts

  # preloadで絞り込み(where)や射影(select)をする場合
  # 事前に関連テーブルを固定的に絞り込む必要があるため柔軟性がない
  # 射影を使うことでカバリングインデックスによる高速化も狙える
  has_many :remarkable_posts, -> { where(posts: { title: 'Post 2' }) }, class_name: 'Post'

  # user.postsに関する情報を関連を含めて出力する
  def full_describe
    posts.each do |post|
      post.describe(user: self)
    end
  end

  # ある特定のタイトルを持つuser.postsに関する情報を関連を含めて出力する
  def partial_describe(title_to_find:)
    posts.filter{ |post| post.title == title_to_find }.each do |post|
      post.describe(user: self)
    end
  end

  # remarkable_postsという特化した関連を用いて絞り込んだ情報を関連を含めて出力する
  def remarkable_describe()
    remarkable_posts.each do |post|
      post.describe(user: self)
    end
  end
end


class Post < ActiveRecord::Base
  belongs_to :person
  has_many :post_comments

  # 関連テーブルを含めてpostに関する情報を出力する
  def describe(user:)
    post_comments.each do |comment|
      comment.post_comment_reviews.each do |review|
        puts "#{user.name}, #{title}, #{comment.comment}, #{review.grade}"
      end
    end
  end
end

# ポストに対するコメント
class PostComment < ActiveRecord::Base
  belongs_to :post
  has_many :post_comment_reviews
end

# コメントに対する評価
class PostCommentReview < ActiveRecord::Base
  belongs_to :post_comment
end

def log(msg)
  ActiveRecord::Base.logger.info(msg)
end

class SqlTest < Minitest::Test
  def test_for_article
    # テストデータをsetup
    # 今回の例ではhas_many関連ごとに3つのデータを作成するが、実環境ではもっとデータが多いとイメージしてほしい
    user = User.create(name: 'Alice')
    1.upto(3) do |i|
      post = user.posts.create(title: "Post #{i}")
      1.upto(3) do |j|
        post.post_comments.create(comment: "Comment #{j}")
        1.upto(3) do |k|
          post.post_comments.last.post_comment_reviews.create(grade: k)
        end
      end
    end
    ActiveRecord::Base.logger = Logger.new($stdout)

    log('テストケース1: キャッシュなし')
    user = User.find_by(name: 'Alice') # sql 1 + 1 + 3 + 3 * 3 = 14、いわゆるN+1問題
    user.full_describe

    log('テストケース2: preloadでキャッシュ')
    user = User.preload(posts: { post_comments: :post_comment_reviews }).find_by(name: 'Alice')
    user.full_describe # sql 1 + 1 + 1 + 1 = 4、スマートなSQL

    log('テストケース3: preload SQLは制御できない')
    user = User.preload(posts: { post_comments: :post_comment_reviews }).find_by(name: 'Alice')
    user.partial_describe(title_to_find: 'Post 2') # 無駄にデータを引いてしまうし、関数も増える

    log('テストケース4: preloadに特化した関連')
    user = User.preload(remarkable_posts: { post_comments: :post_comment_reviews }).find_by(name: 'Alice')
    user.remarkable_describe # 無駄なデータロードはないが、保守しにくい関連も増えるし、それに沿ったデータ呼び出しが必要

    log('テストケース5: APIインタフェースを無視できるなら実は問題は簡単')
    user.posts.preload(post_comments: :post_comment_reviews).where(posts: { title: 'Post 2' }).each { |post|  post.describe(user:) }
    # メソッドを使わなくていいのであれば、当然こう書くこともできる
    # 実装の重要性あるいは不要なインタフェースを定義しないことの重要性を強調したい
    # つまりfull_describeなどのインタフェースは(特にパフォーマンス指向の設計をするにあたり)適切ではない

    log('テストケース6: eager_loadとpreloadの併用')
    user = User.eager_load(:posts).preload(posts: { post_comments: :post_comment_reviews }).where(posts: { title: 'Post 2' }).find_by(name: 'Alice')
    user.full_describe # ちょうどいい、partial_describeも不要、子テーブルはJOIN、孫テーブル以下はpreloadで取得といった制御ができる

    log('テストケース7: includesの意図しない挙動')
    user = User.includes(posts: { post_comments: :post_comment_reviews }).where(posts: { title: 'Post 2' }).find_by(name: 'Alice')
    user.full_describe # 有無も言わさず全部JOINになる、意図せずスローダウンする原因になりうる、includesは使うべきではない

    log('テストケース8: 既存includesのリファクタリング')
    # SQLが発行される直前でeager_loading?を呼び出すと、includesがeager_loadになるかpreloadになるかを判定できる
    # サブテーブルに対するwhere句がある場合は、この絞り込みを処理するために最低でもuserとpostをJOINする必要があるためeager_loadになる
    log(User.includes(posts: { post_comments: :post_comment_reviews }).where(posts: { title: 'Post 2' }).eager_loading?) # true
    # eager_loadが一つでも含まれる場合はひ孫テーブルまで含めてeager_load(JOIN)する
    log(User.eager_load(:posts).includes(posts: { post_comments: :post_comment_reviews }).eager_loading?) # true
    # eager_loadまたは上記絞り込みがない場合はpreloadとして同じになる
    log(User.includes(posts: { post_comments: :post_comment_reviews }).eager_loading?) # false
  end
end
