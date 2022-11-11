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
        t: usize,
        n: usize,
        lrv: [(usize, usize); n]
    };

    let mut cusum = vec![0_i32; t + 1];
    for (l, r) in lrv {
        cusum[l] += 1;
        cusum[r] -= 1;
    }
    for i in 0..t {
        cusum[i + 1] += cusum[i];
    }
    for c in cusum.iter().take(t) {
        println!("{}", c);
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
