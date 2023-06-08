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
        t: usize, n: usize,
        pv: [[usize; n]; t]
    };

    let mut max_v = vec![0; n];
    for ti in 0..t {
        for ni in 0..n {
            max_v[ni] = max_v[ni].max(pv[ti][ni]);
        }
        let res: usize = max_v.iter().sum();
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
