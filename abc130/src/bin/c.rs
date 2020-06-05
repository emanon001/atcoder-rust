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
        w: f64,
        h: f64,
        x: f64,
        y: f64
    };

    let area = w * h / 2.0;
    let is_multiple = x * 2.0 == w && y * 2.0 == h;
    println!("{}", area);
    println!("{}", if is_multiple { 1 } else { 0 });
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
