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
        av: [u64; n]
    };

    let sum = av.iter().sum::<u64>();
    let mut x = sum;
    for i in 0..n / 2 {
        x -= av[i * 2 + 1] * 2;
    }
    let mut res = Vec::new();
    res.push(x);
    for i in 0..n - 1 {
        x = (av[i] * 2) - x;
        res.push(x);
    }
    println!("{}", res.into_iter().join(" "));
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
