#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(n: i128) -> i128 {
    let mut res = 1_i128;
    for i in 0..11 {
        res *= n - i;
    }
    res
}

fn solve() {
    input! {
        l: i128
    };

    let res = f(l - 1) / 39916800;
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
