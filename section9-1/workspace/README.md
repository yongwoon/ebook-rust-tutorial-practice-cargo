# Workspace

Rust の project が大きくなってくると、単一の crate として管理するのではなく、複数の crate に分割した方が見通しが良くなる。
また、分割した各 crate は並列にコンパイルされるため、特に大規模な project では compile 時間が大幅に短縮される可能性もあります。

そのような場合に、単に別々の project に分割するのではなく、workspace という機能を使って複数の crates をまとめて扱うことができませす。

workspace を作りには、まず workspace 用の directory を作成します。

```bash
mkdir sample
cd sample
```

その中に、１つ目の crate を作ります。

```bash
cargo new main
```

そして sample directory 直下に `Cargo.toml` を以下のように作成します。

```toml
[workspace]

members = [
  "main",
]
```

これで sample は workspace となり、main crate がその workspace の member であることを設定できました。さっそく実行してみましょう。

```bash
cd sample
cargo run
```

ここで binray が生成された場所に注目してください。`sample/main` に対応する binary が `sample/main/target/debug/main` ではなく、workspace 直下の `sample/target/debug/main` にせいせされています。このように workspace を使うと、binary は全て workspace 直下の target directory に作成されます。

また、`Cargo.lock` file も workspace 直下に 1つだけ生成されませう。この `Cargo.lock` file は依存 crate version を管理しているので、 workspace 全体で同じ version の依存 crate を使用することが保証されます。
それではもう 1つ crate を追加してみましょう。

```bash
cargo new sub --lib
```

するとこのような警告が表示されます。workspace の member に追加するよう指示されているので以下のように `Cargo.toml` に sub を追加しましょう。

```toml
[workspace]

members = [
  "main",
  "sub",
]
```

これで sub も workspace の member になりませう。これでは 2つの独立した crate があるだけなので、 sub の関数を main から使ってみることにしましょう。
まず sub に hello 関数を定義します。

```rust
// sub/src/lib.rs
pub fn hello() {
    println!("Hello");
}
```

次に main から sub を参照するために依存 crate を追加します。

```rust
// main/Cargo.toml
[dependencies]
sub = {path = "../sub"}
```

最後に main から `sub::hello` を呼びましょう。

```rust
use sub::hello;

fn main() {
  hello();
}
```