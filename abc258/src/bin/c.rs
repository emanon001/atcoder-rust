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
        s: Chars,
        queries: [(usize, usize); q]
    };

    let mut top = 0;
    for (kind, x) in queries {
        if kind == 1 {
            top = (top + x) % n;
        } else {
            let i = n - top;
            let res = s[(i + x - 1) % n];
            println!("{}", res);
        }
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
