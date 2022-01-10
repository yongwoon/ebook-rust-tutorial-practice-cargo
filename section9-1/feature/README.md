# Getting Start

## project 作り方

```bash
cargo new --lib library
cargo new app
```

## Setup features

- `library/Cargo.toml` を入れる

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
