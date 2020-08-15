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
        x: i64, k: i64, d: i64
    };

    let x = x.abs();
    let c = x / d;
    if k <= c {
        let res = (x - (k * d)).abs();
        println!("{}", res);
        return;
    }
    let rest_c = k - c;
    if rest_c % 2 == 0 {
        let res = (x - (c * d)).abs();
        println!("{}", res);
    } else {
        let res = (x - (c * d) - d).abs();
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
