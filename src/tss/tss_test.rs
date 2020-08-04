extern crate test;

use super::*;
use test::Bencher;

#[bench]
fn bench_token(b: &mut Bencher) {
    b.iter(|| 1);
}
