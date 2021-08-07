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
        n: usize, q: usize,
        points: [(i64, i64); n],
        queries: [Usize1; q]
    };

    let points = points.into_iter().map(|(x, y)| (x - y, x + y)).collect::<Vec<_>>();
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    for &(x, y) in &points {
        xv.push(x);
        yv.push(y);
    }
    xv.sort();
    yv.sort();
    let x_min = xv[0];
    let x_max = xv[n - 1];
    let y_min = yv[0];
    let y_max = yv[n - 1];
    for i in queries {
        let (x, y) = points[i];
        let res = std::cmp::max(
            std::cmp::max((x - x_min).abs(), (x - x_max).abs()),
            std::cmp::max((y - y_min).abs(), (y - y_max).abs()),
        );
        println!("{}", res);
    }
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
