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
        sx: i64, sy: i64, gx: i64, gy: i64, t: i64, v: i64,
        n: usize,
        vv: [(i64, i64); n]
    };

    let max_d = t * v;
    let is_ok = vv.into_iter().any(|(x, y)| {
        let d1 = (((sx - x).abs().pow(2) + (sy - y).abs().pow(2)) as f64).sqrt();
        let d2 = (((gx - x).abs().pow(2) + (gy - y).abs().pow(2)) as f64).sqrt();
        (d1 + d2) <= max_d as f64
    });
    let res = if is_ok { "YES" } else { "NO" };
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
