# Section 9-2. formatter, linter

## rustfmt

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
