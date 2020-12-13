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
        mut av: [usize; m]
    };

    av.sort();
    let mut list = Vec::new();
    let mut min = std::usize::MAX;
    let mut prev = 0;
    for &a in &av {
        let w = a - 1 - prev;
        prev = a;
        if w == 0 {
            continue;
        }
        if w < min {
            min = w;
        }
        list.push(w);
    }
    if prev < n {
        let w = n - prev;
        if w < min {
            min = w;
        }
        list.push(w);
    }
    // println!("{}", min);
    // println!("{:?}", list);
    let mut res = 0;
    for w in list {
        res += (w + min - 1) / min;
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
