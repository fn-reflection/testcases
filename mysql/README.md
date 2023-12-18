# MySQL
## 前提
コンテナを立ち上げるとport 53306を通じてMySQLに接続できます。(docker-compose.ymlでポート調整可能です。)
bindディレクトリ以下にdata, logディレクトリがmountされます。

## docker containerの操作
```sh
# UID GIDを設定することで現在のユーザ権限と同じ権限でデータが作られる
UID=$UID GID=$GID docker compose up -d # dockerコンテナ起動
mysql --user root --host 127.0.0.1 --port 53306 -proot # mysql CLIの実行
docker exec -it testcases_mysql /bin/bash # dockerにbashでアクセス
```
