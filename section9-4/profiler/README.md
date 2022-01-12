# cargo-profiler

profiler を呼び出す Cargo の subcommand です。

profiler とは program を実行して、各関数呼び出しの回数を count したり、cache の hit 率などの情報を取得する program です。最適化を行うためにはこのような情報(profile と呼びます)を取得して、注力すべき箇所を特定することが重要です。
`cargo-profiler` 自体は profiler ではなく、外部の profiler として valgrind を利用いています。そのため使用するには `cargo-profiler` の他に valgrind の install が必要です。

```bash
sudo apt-get install valgrind
cargo install cargo-profiler
```

試しに Hello, world の profile をとってみる

```bash
cargo new sample
cd sample
cargo build --release
```

Release build した後、生成された binary を `--bin` option  で指定するだけです。
callgrind では関数の呼び出し回数を取得することができます。
結果は以下のようになります。

```bash
cargo profiler callgrind --bin ./target/release/sample
```

また、cachehit, miss については cachegrind で取得できます。

```bash
cargo profiler cachegrind --bin ./target/release/sample
```
