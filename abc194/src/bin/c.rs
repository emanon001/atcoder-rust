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
        *counts.entry(a).or_insert(0_i64) += 1;
    }

    let max_a = 200;
    let mut res = 0_i64;
    for x in -max_a..max_a  {
        for y in x + 1..=max_a {
            let xc = counts.get(&x).unwrap_or(&0);
            let yc = counts.get(&y).unwrap_or(&0);
            let c = xc * yc;
            res += (x - y).pow(2) * c;
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
