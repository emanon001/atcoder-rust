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
        a: u64, r: u64, n: u64
    };

    if r == 1 {
        println!("{}", a);
        return;
    }

    let mut cur = a;
    for _ in 0..n - 1 {
        cur *= r;
        if cur > 10.pow(9) {
            println!("large");
            return;
        }
    }
    println!("{}", cur);
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
