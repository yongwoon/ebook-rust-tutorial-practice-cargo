# criterion

Criterion は benchmark 用の crate.
`cargo bench` は `nightly` でしか使えないが、こちらは stable でも使える。

```bash
cargo new --lib sample
cargo install -dev criterion
```

## 実行方法

```bash
cd sample
cargo bench
```
