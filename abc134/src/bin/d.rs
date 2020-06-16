#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn divisors(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            res.push(x);
            let y = n / x;
            if y != x {
                res.push(y);
            }
        }
        x += 1;
    }
    res
}

fn f(av: &[usize]) -> Vec<usize> {
    let n = av.len();
    let mut dp = vec![0; n + 1];
    for i in (1..=n).rev() {
        dp[i] ^= av[i - 1];
        for d in divisors(i as u64) {
            let d = d as usize;
            if d == i {
                continue;
            }
            dp[d] ^= dp[i];
        }
    }
    dp
}

fn solve() {
    input! {
        n: usize,
        av: [usize; n]
    };

    let d = f(&av);
    let mut v = Vec::new();
    let mut c = 0;
    for i in 1..=n {
        if d[i] == 1 {
            c += 1;
            v.push(i);
        }
    }
    println!("{}", c);
    println!("{}", v.into_iter().join(" "));
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
