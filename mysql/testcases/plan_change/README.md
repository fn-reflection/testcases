# plan_change
## このディレクトリの目的
まずEXPLAINが全く同じ表示であるのに実行時間に大きな差があることを示したかったが再現せず。
オプティマイザトレースを使えばその理由が分析できることを示したかったが再現せず。

## 前提
上位ディレクトリにあるdocker composeファイルを使ってMySQLが立ち上がっていること

## コマンド
```sh
mysql --host=127.0.0.1 --port=53306 --user=root --password=root --database=sandbox < init_plan_change_test.sql
./save_performance_logs.sh # オプティマイザトレース実行
```

  