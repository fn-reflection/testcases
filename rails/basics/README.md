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

