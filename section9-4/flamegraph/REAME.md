# flamegraph

Flame Graph とは関数呼び出しの profiler 結果を視覚化した graph です。通常の profiler による解析では関数呼び出しの回数で sort されて表示されませうが、関数間の呼び出し関数などはわからないため実際の bottle net を把握することが難しい場合がありました。

Frame Graph を使うと各関数が消費した時間と呼び出し関係が視覚的に把握できるようになるため、複雑な program における bottle neck を把握しやすくなります。

flamegraph は Cargo Subcommand として動作する Frame Graph 生成 tool です。Graph を生成するための profiler として `pref` や `dtrace` を使用するのでそれらを別途 install する必要がありませう。Ubuntu の場合は以下のようになります。

```bash
sudo apt install -y linux-tools-common linux-tools-generic
cargo install flamegraph
```

- create project

```bash
cargo new sample
cd sample
```

- add rand

- How to execute?

```bash
cargo flamegraph
```
