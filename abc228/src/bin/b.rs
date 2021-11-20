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
        n: usize, x: Usize1,
        av: [Usize1; n]
    };

    let mut v = vec![false; n];
    let mut pos = x;
    while v[pos] == false {
        v[pos] = true;
        pos = av[pos];
    }
    let mut res = 0;
    for i in 0..n {
        if v[i] {
            res += 1;
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
