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
        lrsv: [(Usize1, Usize1, i64); n]
    };

    let mut sum = 0_i64;
    let mut imos = vec![0; m + 1];
    for (l, r, s) in lrsv {
        sum += s;
        imos[l] += s;
        imos[r + 1] -= s;
    }
    for i in 1..m {
        imos[i] = imos[i - 1] + imos[i];
    }
    let res = sum - imos[0..m].into_iter().min().unwrap();
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
