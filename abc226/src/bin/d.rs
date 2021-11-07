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
        points: [(i64, i64); n]
    };

    let mut set = HashSet::new();
    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let dx = x2 - x1;
            let dy = y2 - y1;
            let gcd = dx.gcd(&dy);
            let d = (dx / gcd, dy / gcd);
            set.insert(d);
        }
    }
    for i in (0..n).rev() {
        for j in (0..i).rev() {
            let (x1, y1) = points[i];
            let (x2, y2) = points[j];
            let dx = x2 - x1;
            let dy = y2 - y1;
            let gcd = dx.gcd(&dy);
            let d = (dx / gcd, dy / gcd);
            set.insert(d);
        }
    }
    println!("{}", set.len());
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
