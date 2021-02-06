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
        a: i64, b: i64, c: usize
    };

    let res = if c == 0 {
        if a > b {
            "Takahashi"
        } else {
            "Aoki"
        }
    } else {
        if b > a {
            "Aoki"
        } else {
            "Takahashi"
        }
    };
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
