#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        mut pv: [(i64, i64); n]
    };

    // x
    pv.sort_by_key(|p| p.0);
    let mid = pv[n / 2];
    let mut sum = 0;
    for p in pv.iter().copied() {
        sum += (mid.0 - p.0).abs();
    }
    // y
    pv.sort_by_key(|p| p.1);
    let mid = pv[n / 2];
    for p in pv.iter().copied() {
        sum += (mid.1 - p.1).abs();
    }
    println!("{}", sum);
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
