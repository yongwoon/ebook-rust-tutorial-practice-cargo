# rust-tutorial-practice-cargo

実践 Rust programing 入門 - section 9

## section 9-1

### 順番

- dir
  - 여러가지 기능별로 폴더 구성
  - `section9-1/dir`
- feature
  - `feature` 의 이해 및 활용
  - `section9-1/feature`
- patch
  - 既存 crate の修正
  - `section9-1/patch`
- workspace
  - 風数の crate に分割したい場合にする。

## Section 9-2, 9-3. formatter, linter

### rustfmt

rustfmt は Rust 標準の source code・formatter です。
rustfmt を使うことでその style を自動的に適用することができます。
具体的なスタイルの詳細は以下のようにまとめられます。

- Install

```bash
rustup component add rustfmt
```

- 実行方法

```bash
cargo fmt
cargo fmt --check # 差分表示
```

- Link
  - [Rust Style Guide](https://github.com/rust-dev-tools/fmt-rfcs/blob/master/guide/guide.md)
  - [rustfmt の設定](https://github.com/rust-lang/rustfmt#running-rustfmt-from-your-editor)
  - [clippy](https://github.com/rust-lang/rust-clippy)
    - 標準の rust lint checker
  - Code Coverage
    - [cargo-tarpaulin](https://github.com/xd009642/tarpaulin)

## Section 9-4

### benchmark

- Create project

```bash
cargo new --lib sample
```

- 実行方法

```bash
cargo +nightly bench
```

### criterion

Criterion は benchmark 用の crate.
`cargo bench` は `nightly` でしか使えないが、こちらは stable でも使える。

```bash
cargo new --lib sample
cargo install -dev criterion
```

- 実行方法

```bash
cd sample
cargo bench
```

### cargo-profiler

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


## Link

- [本のテストコード](https://github.com/forcia/rustbook)
