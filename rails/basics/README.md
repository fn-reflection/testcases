# Rails Basics
## 前提
このリポジトリの/mysqlで`docker compose up -d`を実行しコンテナを立ち上げていること
- `mysql_default`と言うdocker networkがある
- コンテナが正常稼働している

## docker containerの操作
```sh
UID="$(id -u)" GID="$(id -g)" docker compose up -d # 現在のユーザ権限を維持しながらdockerコンテナ起動
docker exec -it rails_basics /bin/bash # dockerにbashでアクセス
```

## docker container内での操作
```sh
cd myapp # rails appのディレクトリに移動
bundle exec rails s -b 0.0.0.0 # developmentモードでサーバ起動(全てのIPからの接続を待ち受ける)
EDITOR="nano" bundle exec rails credentials:edit # productionモードで必要となるcredentialファイル(暗号化情報)をでっち上げる
RAILS_ENV=production bundle exec rails s -b 0.0.0.0 # productionモードでサーバ起動(全てのIPからの接続を待ち受ける)

```


