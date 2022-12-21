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
        av: [i64; n]
    };

    let mut map = HashMap::new();
    for a in av {
        *map.entry(a).or_insert(0_i64) += 1;
    }
    let res = map.into_iter().fold(0_i64, |acc, (_, c)| {
        acc + if c < 3 { 0 } else { c * (c - 1) * (c - 2) / 6 }
    });
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
