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
        n: usize, m: usize,
        abv: [(usize, usize); n]
    };

    let mut works = vec![Vec::new(); m + 1];
    for (a, b) in abv {
        if a <= m {
            works[a].push(b);
        }
    }
    let mut heap = BinaryHeap::new();
    let mut res = 0_usize;
    for i in 1..=m {
        for &b in &works[i] {
            heap.push(b);
        }
        if let Some(b) = heap.pop() {
            res += b;
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
