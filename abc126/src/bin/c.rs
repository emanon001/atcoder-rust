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
        n: usize, k: usize,
    };

    let mut res = 0_f64;
    for a in 1..n + 1 {
        let mut c = 0;
        while a * 2.pow(c) < k {
            c += 1;
        }
        res += (1.0 / n as f64) * (1_f64 / 2.pow(c as u32) as f64);
    }
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
