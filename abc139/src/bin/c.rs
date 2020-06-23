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
        hv: [usize; n]
    };

    let mut res = 0;
    let mut prev = hv[0];
    let mut c = 0;
    for h in hv.into_iter().skip(1).chain(vec![std::usize::MAX]) {
        if prev >= h {
            c += 1;
        } else {
            res = std::cmp::max(res, c);
            c = 0;
        }
        prev = h;
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
