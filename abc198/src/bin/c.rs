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
        r: f64, x: f64, y: f64
    };

    let d = (x * x + y * y).sqrt();
    let res = if d == r {
        1.0
    } else if d < r {
        2.0
    } else {
        (d / r).ceil()
    };
    println!("{}", res as i64);
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
