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
  create_table :users do |t|
    t.text :name
  end

  create_table :posts do |t|
    t.integer :user_id
    t.text :title
  end

  create_table :books do |t|
    t.integer :user_id
    t.text :title
  end

  create_table :articles do |t|
    t.integer :user_id
    t.text :title
  end

  create_table :documents do |t|
    t.integer :user_id
    t.text :title
  end
end

# モデル定義

# ユーザ
class User < ActiveRecord::Base
  has_many :posts
  has_many :books
  has_many :articles, inverse_of: :author
  has_many :documents
end

class Post < ActiveRecord::Base
  belongs_to :user
end

class Book < ActiveRecord::Base
  belongs_to :author, foreign_key: 'user_id', class_name: 'User'
end

class Article < ActiveRecord::Base
  belongs_to :author, foreign_key: 'user_id', class_name: 'User'
end

class Document < ActiveRecord::Base
  belongs_to :user, foreign_key: 'user_id', class_name: 'User'
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
      user.posts.create(title: "Post #{i}")
      user.books.create(title: "Book #{i}")
      user.articles.create(title: "Article #{i}")
      user.documents.create(title: "Document #{i}")
    end
    ActiveRecord::Base.logger = Logger.new($stdout)

    user = User.find_by(name: 'Alice')
    user2 = user.posts.first.user
    assert(user.equal?(user2)) # キャッシュが効く

    user = User.find_by(name: 'Alice')
    user2 = user.books.first.author
    assert(user.object_id != user2.object_id) # キャッシュが効いていない

    user = User.find_by(name: 'Alice')
    user2 = user.articles.first.author # キャッシュが効く
    assert(user.equal?(user2))

    user = User.find_by(name: 'Alice')
    user2 = user.documents.first.user # foreign_keyを指定するか否かが要
    assert(user.object_id != user2.object_id)
  end
end
