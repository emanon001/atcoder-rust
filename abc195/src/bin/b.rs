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
        a: usize, b: usize, w: usize
    };

    let w = w * 1000;
    let min = (w + b - 1) / b;
    let max = w / a;
    if min > max {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", min, max);
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
