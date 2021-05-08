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
        av: [usize; n]
    };

    let mut counts = vec![0; 200];
    for a in av {
        counts[a % 200] += 1;
    }
    let mut res = 0_i64;
    for x in 0..200 {
        let c = counts[x];
        if c > 0 {
            res += c * (c - 1);
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
