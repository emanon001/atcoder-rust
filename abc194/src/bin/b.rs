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
        abv: [(i64, i64); n]
    };

    let mut res = i64::max_value();
    for i in 0..n {
        for j in 0..n {
            let time = if i == j {
                abv[i].0 + abv[j].1
            } else {
                (abv[i].0).max(abv[j].1)
            };
            res = res.min(time);
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
