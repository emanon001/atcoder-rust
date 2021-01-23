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
        n: usize,
        mut abv: [(i64, i64); n]
    };

    abv.sort_by_key(|(a, b)| -((a + b) + a));
    let mut score = 0;
    for &(a, _) in &abv {
        score -= a;
    }
    let mut c = 0;
    for &(a, b) in &abv {
        score += a + (a + b);
        c += 1;
        if score > 0 {
            println!("{}", c);
            return;
        }
    }
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
