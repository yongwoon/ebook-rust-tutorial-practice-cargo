# hyperfine

benchmark を取る際に便利な tool として hyperfine があります。特に Cargo との連携などはありませんが、いくつかの command line program を複数回実行して総計情報を取るなどの操作を手軽に行うことができます。
例えば以下のように install して実行します。

```bash
cargo install hyperfine
```

```bash
hyperfine "program1 data.txt" "program2 data.txt"
```
