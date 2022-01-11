// 実行方法
// cargo +nightly bench
// 
// benchmark 機能は nightly 限定なので、
// #![feature(test)] によってその仕様を明示する必要があります。
#![feature(test)]

pub fn hash(x: u64) -> u64 {
    let mut y = x;
    for _ in 0..512 {
        y = y << 5;
        y = y ^ x;
    }
    y
}
