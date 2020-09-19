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
        c: usize,
        nmlv: [[usize; 3]; c]
    };

    let (a, b, c) = nmlv.into_iter().fold((0, 0, 0), |acc, mut v| {
        v.sort();
        (acc.0.max(v[0]), acc.1.max(v[1]), acc.2.max(v[2]))
    });
    let res = a * b * c;
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
