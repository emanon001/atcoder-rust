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
        n: usize, x: u64,
        wv: [u64; n]
    };

    let mut dp1: HashMap<u64, u64> = HashMap::new();
    dp1.insert(0, 1);
    for i in 0..n / 2 {
        let w = wv[i];
        let mut new_dp = HashMap::new();
        for (&v, &c) in &dp1 {
            *new_dp.entry(v).or_insert(0) += c;
            *new_dp.entry(v + w).or_insert(0) += c;
        }
        dp1 = new_dp;
    }
    let mut dp2: HashMap<u64, u64> = HashMap::new();
    dp2.insert(0, 1);
    for i in n / 2..n {
        let w = wv[i];
        let mut new_dp = HashMap::new();
        for (&v, &c) in &dp2 {
            *new_dp.entry(v).or_insert(0) += c;
            *new_dp.entry(v + w).or_insert(0) += c;
        }
        dp2 = new_dp;
    }
    let mut res = 0_u64;
    for (v, c) in dp1 {
        if v > x {
            continue;
        }
        if let Some(&c2) = dp2.get(&(x - v)) {
            res += c * c2;
        }
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
