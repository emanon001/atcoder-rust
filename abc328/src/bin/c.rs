#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize, q: usize,
        s: Chars
    };

    let mut cusum = vec![0; n + 1];
    for i in 0..n - 1 {
        let c1 = s[i];
        let c2 = s[i + 1];
        cusum[i + 1] = cusum[i] + if c1 == c2 { 1 } else { 0 };
    }
    for _ in 0..q {
        input_interactive! {
            l: usize, r: usize
        };
        let ans = cusum[r - 1] - cusum[l - 1];
        println!("{}", ans);
    }
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
