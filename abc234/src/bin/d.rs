#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, k: usize,
        pv: [i64; n]
    };

    let mut res = Vec::new();
    let mut heap = BinaryHeap::new();
    for i in 0..k {
        heap.push(Reverse(pv[i]));
    }
    let Reverse(x) = *heap.peek().unwrap();
    println!("{}", x);
    for i in k..n {
        let p = pv[i];
        let Reverse(x) = *heap.peek().unwrap();
        if p > x {
            heap.pop();
            heap.push(Reverse(p));
        }
        let Reverse(x) = *heap.peek().unwrap();
        res.push(x);
    }
    println!("{}", res.iter().join("\n"));
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
