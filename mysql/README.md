# MySQL
## 前提
コンテナを立ち上げるとport 53306を通じてMySQLに接続できます。(docker-compose.ymlでポート調整可能です。)
これはあくまでテスト用なのでデータの永続化はされません。

## docker containerの操作
```sh
docker compose up -d # dockerコンテナ起動
mysql --user root --host 127.0.0.1 --port 53306 -proot # mysql CLIの実行
docker exec -it testcases_mysql /bin/bash # dockerにbashでアクセス
```