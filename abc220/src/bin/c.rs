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
        av: [u64; n],
        x: u64
    };

    let sum: u64 = av.iter().sum();
    let c = x / sum;
    let mut res = c * n as u64;
    let mut cur = c * sum;
    for i in 0..n {
        if cur > x {
            break;
        }
        cur += av[i];
        res += 1;
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
