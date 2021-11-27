#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, w: i64,
        mut abv: [(i64, i64); n]
    };

    abv.sort_by_key(|(a, _)| -a);
    let mut sum = 0_i64;
    let mut count = 0_i64;
    for i in 0..n {
        let (a, b) = abv[i];
        sum += (w - count).min(b) * a;
        count += (w - count).min(b);
    }
    println!("{}", sum);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
