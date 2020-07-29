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
        av: [usize; n]
    };

    let mut res = 0;
    let mut c = 0;
    let mut prev = 0;
    for a in av {
        if a > prev {
            c += 1;
        } else {
            c = 1;
        }
        if c >= k {
            res += 1;
        }
        prev = a;
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
