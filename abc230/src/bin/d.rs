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
        n: usize, d: i64,
        mut lrv: [(i64, i64); n]
    };

    lrv.sort_by_key(|&(_, r)| r);

    let mut res = 0;
    let mut pos = -1;
    for (l, r) in lrv {
        if l <= pos {
            continue;
        }
        res += 1;
        pos = r + d - 1;
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
