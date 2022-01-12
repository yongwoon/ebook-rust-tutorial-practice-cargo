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


## Link

- [本のテストコード](https://github.com/forcia/rustbook)
