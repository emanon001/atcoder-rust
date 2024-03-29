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
        n: usize, d: usize,
        mut xyv: [(usize, i64); n]
    };

    xyv.sort_by_key(|(x, y)| (-*y, *x));
    let mut used = vec![false; d + 1];
    let mut res = 0;
    for (x, y) in xyv {
        for i in x..=d {
            if !used[i] {
                used[i] = true;
                res += y;
                break;
            }
        }
    }
    println!("{}", res);
}

fn solve_pq() {
    input! {
        n: usize, d: usize,
        mut xyv: [(Usize1, i64); n]
    };

    let mut jobs = vec![Vec::new(); d + 1];
    for (x, y) in xyv {
        jobs[x].push(y);
    }
    let mut res = 0;
    let mut heap = BinaryHeap::new();
    for day in 0..d {
        for &p in &jobs[day] {
            heap.push(p);
        }
        res += heap.pop().unwrap_or(0);
    }
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve_pq();
        })
        .unwrap()
        .join()
        .unwrap();
}
