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
        mut av: [i64; n]
    };
    av.sort();
    let mut cusum = Vec::new();
    let mut prev = 0;
    for i in 0..n {
        prev += av[i];
        cusum.push(prev);
    }

    let mut res = 0_i64;
    for i in 0..n - 1 {
        let add = av[i] * (n - i - 1) as i64;
        let sub = cusum[i + 1];
        res += (add - sub).abs();
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
