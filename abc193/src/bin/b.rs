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
        apvx: [(i64, i64, i64); n]
    };

    let mut res = 1_i64 << 60;
    for (a, p, x) in apvx {
        if x <= a {
            continue;
        }
        res = res.min(p);
    }
    let res = if res == 1_i64 << 60 { -1 } else { res };
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
