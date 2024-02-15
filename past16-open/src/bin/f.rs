#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        S: String
    };

    let MOD = 998244353_i64;

    let ans = S.split('*').fold(1_i64, |acc, x| {
        let mut n = 0_i64;
        for x in x.chars().map(|ch| ch.to_digit(10).unwrap() as i64) {
            n = (n * 10 + x) % MOD;
        }
        (acc * n) % MOD
    });
    println!("{}", ans);
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
