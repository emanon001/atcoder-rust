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
        n: i128, a: i128, b: i128
    };

    let a_count = n / a;
    let b_count = n / b;
    let ab = a.lcm(&b);
    let ab_count = n / ab;
    let res = ((1 + n) * n / 2)
        - (a * ((1 + a_count) * a_count / 2))
        - (b * ((1 + b_count) * b_count / 2))
        + (ab * ((1 + ab_count) * ab_count / 2));
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
