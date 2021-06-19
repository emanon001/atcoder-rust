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

    let mut counts = HashMap::new();
    for a in av {
        *counts.entry(a).or_insert(0) += 1;
    }

    let mut res = 0_i64;
    for (_, c) in counts {
        let d = n as i64 - c;
        if d >= 1 {
            res += c * d;
        }
    }
    println!("{}", res / 2);
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
