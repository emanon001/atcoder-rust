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
        xyv: [(i64, i64); n]
    };

    let mut points = Vec::new();
    for (x, y) in xyv {
        points.push((x + y, x - y));
    }
    points.sort_by_key(|p| p.0);
    let mut res = (points[0].0 - points[n - 1].0).abs();
    points.sort_by_key(|p| p.1);
    res = res.max((points[0].1 - points[n - 1].1).abs());
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
