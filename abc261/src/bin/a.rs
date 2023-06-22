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
        l: usize, r: usize, l2: usize, r2: usize
    };

    let mut v = vec![0; 100 + 1];
    for i in l + 1..=r {
        v[i] += 1;
    }
    for i in l2 + 1..=r2 {
        v[i] += 1;
    }
    let res = v.into_iter().filter(|c| *c == 2).count();
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
