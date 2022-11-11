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
        d: usize,
        n: usize,
        lrv: [(Usize1, Usize1); n]
    };

    let mut cusum = vec![0_i32; d + 1];
    for (l, r) in lrv {
        cusum[l] += 1;
        cusum[r + 1] -= 1;
    }
    for i in 0..d {
        cusum[i + 1] += cusum[i]
    }
    for c in cusum.iter().take(d) {
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
