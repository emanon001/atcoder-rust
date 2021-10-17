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
        abv: [(f64, f64); n]
    };
    let mut t = 0_f64;
    for &(a, b) in &abv {
        t += a / b;
    }
    t /= 2_f64;
    let mut res = 0_f64;
    for &(a, b) in &abv {
        res += (a / b).min(t) * b;
        t -= (a / b).min(t);
    }
    println!("{}", res);
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
