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
        l: usize, r: usize
    };

    let m = 2019;
    let r = std::cmp::min(l + 2019 - 1, r);
    let mut res = m;
    for a in l..r {
        for b in a + 1..=r {
            let c = (a * b) % m;
            if c < res {
                res = c;
            }
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
