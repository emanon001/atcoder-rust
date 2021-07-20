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
        queries: [(Usize1, Usize1, i64); q]
    };

    let mut diff = vec![0; n - 1];
    let mut res = 0_i64;
    for i in 0..n - 1 {
        diff[i] = av[i] - av[i + 1];
        res += diff[i].abs();
    }
    for (l, r, v) in queries {
        if l > 0 {
            let x = diff[l - 1];
            diff[l - 1] -= v;
            let y = diff[l - 1];
            if y.abs() > x.abs() {
                res += y.abs() - x.abs();
            } else {
                res -= x.abs() - y.abs();
            }
        }
        if r < n - 1 {
            let x = diff[r];
            diff[r] += v;
            let y = diff[r];
            if y.abs() > x.abs() {
                res += y.abs() - x.abs();
            } else {
                res -= x.abs() - y.abs();
            }
        }
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
