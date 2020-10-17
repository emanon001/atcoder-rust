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
        xv: [f64; n]
    };

    let a = xv.iter().map(|x| x.abs()).sum::<f64>();
    let b = xv.iter().map(|x| x.abs() * x.abs()).sum::<f64>().sqrt();
    let c = xv.iter().map(|x| x.abs() as u64).max().unwrap();
    println!("{} {} {}", a, b, c);
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
