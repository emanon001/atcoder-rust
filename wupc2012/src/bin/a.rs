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
        ma: usize, da: usize,
        mb: usize, db: usize,
    };

    let days = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let res = if ma == mb {
        db - da
    } else {
        let mut res = 0;
        res += days[ma - 1] - da;
        for m in ma + 1..mb {
            res += days[m - 1];
        }
        res += db;
        res
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
