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
        lrtv: [(Usize1, Usize1, usize); q]
    };

    let mut v = vec![0; n];
    for (l, r, t) in lrtv {
        for i in l..r + 1 {
            v[i] = t;
        }
    }
    for i in 0..n {
        println!("{}", v[i]);
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
