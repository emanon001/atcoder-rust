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
        av: [i64; n],
        lrv: [(usize, usize); q]
    };

    let mut cusum = vec![0; n + 1];
    for i in 0..n {
        cusum[i + 1] = cusum[i] + av[i];
    }

    for (l, r) in lrv {
        let res = cusum[r] - cusum[l - 1];
        println!("{}", res);
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
