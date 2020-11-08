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
        av: [i64; n]
    };

    let mut cusum = vec![0_i64; n + 5];
    let mut cumax = vec![std::i64::MIN; n + 5];
    for i in 1..=n {
        cusum[i] = cusum[i - 1] + av[i - 1];
        cumax[i] = cumax[i - 1].max(cusum[i]);
    }
    let mut res = 0_i64;
    let mut cur = 0_i64;
    for x in 1..=n {
        let t = cur + cumax[x];
        res = res.max(t);
        cur += cusum[x];
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
