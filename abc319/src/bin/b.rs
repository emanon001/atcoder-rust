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
    };

    let res = (0..=n)
        .map(|i| {
            (1..=9)
                .filter(|j| n % j == 0 && i % (n / j) == 0)
                .min()
                .map(|j| j.to_string())
                .unwrap_or("-".to_string())
        })
        .join("");
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
