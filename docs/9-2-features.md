# features

feature とは crate に option の機能を提供させるための仕組みです。
例えばごく少数(아주 적은)の user だけが必要とする機能を feature として提供することで、大多数の user はその機能にまつわるコードを compile せずにすみ、compile 時間の短縮や binary size の証言が見込めます。

さっそく feature を使ってみましょう。今回は

- feature を提供する crate library と
- それを使う crate app

という構成とします。

```bash
cargo new --lib library
cargo new app
cd library
```

`library/Cargo.toml` の `[features]` section に feature を定義することができます。

```toml
# library/Cargo.toml

[features]
# default で有効になる feature
default = ["paralle", "serde"]

# 依存関係を持たない feature
special = []

# 依存 crate を持つ feature
parallel = ["crossbeam"]

# 依存 crate の feature を有効にする feature
nightly = ["crossbeam/nightly"]

# いろいろな依存関係を混ぜた例
full = ["crossbeam/nightly", "parallel", "serde"]

[dependencies]
crossbeam = {version = "0.7.3", optional = true}
serde = {version = "1.0.111", optional = true}
```

まず、feature 名は `[dependencies]` section の crate 名と名前空間を共有しています。そのため crate 名と同じ feature 名をつけることはできませんし、crate 名は feature 名としても機能します。

これを踏まえて(근거로 하다, 입각하다.) `[features]` section をみていきませよう。まず `default` は特別な feature 名で、右辺の配列に default を有効になる feature を列挙します。ここでは `parallel` という feature と `serde` という feature を有効しています。`serde` は `[dependencies]` section で定義した依存 crate 名ですが、このように feature 名としても使うことができます。

special, parallel, nightly, full はそれぞれ新たに定義した feature 名で、その右辺値はその feature が有効になったときに同時に有効になるものの配列を取ります。例えば空配列 `[]` はその feature が何の依存館家も持たないことを示します。
feature 名の右辺値には、feature 名、crate 名の他に `crossbeam/nightly`ように`/` で区切ったものを与えることができます。これは `crossbeam` crate の nightly feature を指します。
すなわち library crate の nightly feature を有効にすると、同時に `crossbeam` crate の nightly feature も有効になる、とういうことを意味しています。

full feature のように crate 名や feature 名、`/` 区切り気泡などはませて使用することができます。
`[dependencies]` を見ると、`optional = true`という記述があります。これは optional な依存関係を表しており、feature によって依存するかどうかを切り替えたい場合に指定します。
これらの feature を使ったコードを書いてみませよう。

```rust
// library/src/lib.rs
#[cfg(feature = "parallel")]
pub fn parallel() {
  println("parallel is enabled");
}

#[cfg(feature = "serde")]
pub fn serde() {
  println!("serde is enabled");
}

#[cfg(feature = "special")]
pub fn special() {
  println!("special is enabled");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parallel() {
    parallel();
  }

  #[test]
  fn test_serde() {
    serde();
  }

  #[test]
  fn test_special() {
    special();
  }
}
```

## How to execute ?

```bash
cd section9-1/feature

cargo test 
# special feature を有効にして TEST
cargo test --features special
# default feature を全て無視して TEST する
cargo test --no-default-features
```
