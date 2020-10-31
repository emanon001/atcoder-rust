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
        a: u64, b: u64, c: u64
    };

    let m = 998244353_u64;
    let a = ((1 + a) * a / 2) % m;
    let b = ((1 + b) * b / 2) % m;
    let c = ((1 + c) * c / 2) % m;
    let res = (((a * b) % m) * c) % m;
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
