// 実行方法
// cargo +nightly bench
// 
// benchmark 機能は nightly 限定なので、
// #![feature(test)] によってその仕様を明示する必要があります。
#![feature(test)]

extern crate test;

use sample::hash;
use test::Bencher;


#[bench]
fn bench_hash(b: &mut Bencher) {
    // 実際に実行したい code はその Bencher の iter method に closer で渡します
    b.iter( || {
        // test::black_box() は正しく benchmark を行うために最適化を阻害(저해)することを示しています。
        // もし AAA のように書いたとすると、Compile 時の最適化により `hash(2)`は
        // その計算結果の固定値に置き換えられてしまい、実際の計算時間が求められません。
        // test::black_box を経由して引数を渡すことで、その引数を最適化に含めないようにできます。
        let n = test::black_box(2);
        hash(n)
    });
}



// AAA
// #[bench]
// fn bench_hash(b: &mut Bencher) {
//     b.iter( || {
//         hash(2)
//     });
// }