#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(av: &[usize]) -> Vec<usize> {
    let n = av.len();
    let mut dp = vec![0; n + 1];
    for i in (1..=n).rev() {
        let mut bit = 0;
        let mut m = 2;
        while i * m <= n {
            bit ^= dp[i * m];
            m += 1;
        }
        dp[i] = bit ^ av[i - 1];
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
