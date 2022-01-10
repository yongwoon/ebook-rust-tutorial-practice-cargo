# Patch (既存 crates の修正)

## Getting start

```bash
cargo new sample
cd sample
cargo install uuid
```

- ここで依存している uuid crate に何らかの修正を加えることにします。そのためにまず uuid の repository を clone してくれる必要があります

```bash
cd ..
git clone https://github.com/uuid-rs/uuid.git
```

- sample directory と同じ階層に uuid の directory を clone してきます。さらに `sample/Cargo.toml` に以下のように追記します。

```toml
[patch.crates-io]
uuid = {path = "../uunid"}
```

- この状態で compile すると以下のようになります。

```bash
cd sample
cargo run
```
