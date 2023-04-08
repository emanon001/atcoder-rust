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
        n: usize, k: usize, x: i64,
        av: [i64; n]
    };

    let mut rest_count = k as i64;
    let mut av2 = Vec::new();
    for a in av {
        let c = (a / x).min(rest_count);
        av2.push(a - x * c);
        rest_count -= c;
    }
    av2.sort_by(|a, b| b.cmp(a));
    let mut res = 0_i64;
    for a in av2 {
        let c = ((a + x - 1) / x).min(rest_count);
        if a > c * x {
            res += a - c * x;
        }
        rest_count -= c;
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
