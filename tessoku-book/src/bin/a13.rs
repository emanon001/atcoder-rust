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
        n: usize, k: i64,
        av: [i64; n]
    };

    let mut rv = vec![0; n];
    for i in 0..n {
        rv[i] = (if i == 0 { 0 } else { rv[i - 1] }).max(i);
        while rv[i] + 1 < n && av[rv[i] + 1] - av[i] <= k {
            rv[i] += 1;
        }
    }
    let mut res = 0;
    for i in 0..n {
        res += rv[i] - i;
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
