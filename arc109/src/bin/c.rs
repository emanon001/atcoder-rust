#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(pos: usize, d: usize, n: usize, k: usize, s: &[char], dp: &mut [Vec<Option<char>>]) -> char {
    if d == k {
        return s[pos];
    }
    if let Some(res) = dp[pos][d] {
        return res;
    }
    let a = f((pos * 2) % n, d + 1, n, k, s, dp);
    let b = f((pos * 2 + 1) % n, d + 1, n, k, s, dp);
    let res = match (a, b) {
        ('R', 'S') => 'R',
        ('S', 'R') => 'R',
        ('P', 'R') => 'P',
        ('R', 'P') => 'P',
        ('S', 'P') => 'S',
        ('P', 'S') => 'S',
        _ => a,
    };
    dp[pos][d] = Some(res);
    return res;
}

fn solve() {
    input! {
        n: usize, k: usize,
        s: Chars
    };

    let mut dp = vec![vec![None; k + 1]; n];
    let res = f(0, 0, n, k, &s, &mut dp);
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
