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
        a: usize,
        mut b: usize,
        mut c: usize,
        k: usize
    };

    let mut rest = k;
    while rest > 0 && b <= a {
        b *= 2;
        rest -= 1;
    }
    while rest > 0 && c <= b {
        c *= 2;
        rest -= 1;
    }
    let res = if a < b && b < c { "Yes" } else { "No" };
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
