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
        a: f64, b: f64, d: f64
    };

    let rad = d.to_radians();

    let a2 = a * rad.cos() - b * rad.sin();
    let b2 = a * rad.sin() + b * rad.cos();
    println!("{} {}", a2, b2);
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
