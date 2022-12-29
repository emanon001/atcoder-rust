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
        av: [Usize1; n],
        xyv: [(Usize1, i64); q]
    };

    let mut doubling = Vec::new();
    doubling.push(av);
    for _ in 0..30 {
        let prev = doubling.last().unwrap();
        let av = prev.iter().copied().map(|a| prev[a]).collect::<Vec<_>>();
        doubling.push(av);
    }
    for (x, y) in xyv {
        let mut pos = x;
        let mut rest = y;
        for d in (0..30).rev() {
            if rest & 2.pow(d) != 0 {
                rest -= 2.pow(d);
                pos = doubling[d as usize][pos];
            }
        }
        println!("{}", pos + 1);
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
