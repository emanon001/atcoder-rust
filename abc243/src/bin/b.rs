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
        av: [i64; n],
        bv: [i64; n],
    };

    let mut res1 = 0;
    for i in 0..n {
        if av[i] == bv[i] {
            res1 += 1;
        }
    }
    println!("{}", res1);

    let mut res2 = 0;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            if av[i] == bv[j] {
                res2 += 1;
            }
        }
    }
    println!("{}", res2);
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
