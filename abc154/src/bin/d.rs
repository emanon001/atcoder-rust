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
        n: usize, k: usize,
        pv: [usize; n]
    };

    let cusum = vec![0_f64]
        .into_iter()
        .chain(pv.into_iter().scan(0_f64, |acc, p| {
            *acc += ((1 + p) * p) as f64 / 2_f64 / p as f64;
            Some(*acc)
        }))
        .collect::<Vec<_>>();
    let mut res = 0_f64;
    for i in 0..n {
        if i + k > n {
            continue;
        }
        let sum = cusum[i + k] - cusum[i];
        if sum > res {
            res = sum;
        }
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
