```sh
docker compose up -d # dockerコンテナ起動(なければボリュームも生成)
mysql --user root --host 127.0.0.1 --port 53306 -proot # mysql CLIの実行
docker exec -it testcases_mysql /bin/bash # dockerにbashでアクセス
```