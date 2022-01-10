# Directory structure

## src/bin

追加 binary を生成する場合に使用します。
default では `src/main.rs` から `Cargo.toml` の package 名と同じ名前の binary が生成されます。

ここで `src/bin/single_bin.rs` を配置すると `single_bin` という名前の binary も同時に生成されるようになります。

また、`src/bin` 以下に directory を作成することでも追加の binary を生成することができます。例えば `src/bin/multi_bin/main.rs` からは `multi_bin` という名前の binary が生成されます。

## tests

`tests` は test を配置するために directory.
主に project 全体の**結合 test**を配置する際に使われます。

`src/bin` と同じく単一 file と directory のどちらかを選択可能となっており、file 名あるいは directory 名が test 名となります。

例えば `tests/single_test.rs` と `tests/multi_test` のテストはそれぞれ以下のように実行可能です。(単に cargo test とすれば src や tests を含めた全てのテストが実行されます)

```bash
cargo test --test single_test
cargo test --test multi_test
```

## examples

examples は使用例を配置するための directory です。
File ・ Directory 構造はこれまでと同様です。 examples に配置した使用例は
`cargo run --example` で実行することができます。

例えば `examples/single_example.rs` と `example/miulti_example` はそれぞれ以下のように実行可能です。

```bash
cargo run --example single_example
cargo run --example multi_example
```

## benches

benches は benchmark 用の source code を配置するための directory です。
benchmark は src などに配置せずにこの directory に配置するのがおすすめです。
単体での実行方法もこれまでと同様です。

```bash
cargo bench --bench single_bench
cargo bench --bench multi_bench
```
