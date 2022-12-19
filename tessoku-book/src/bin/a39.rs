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
        mut lrv: [(i64, i64); n]
    };

    lrv.sort_by_key(|(_, r)| *r);
    let mut res = 0;
    let mut cur = 0;
    for (l, r) in lrv {
        if l >= cur {
            res += 1;
            cur = r;
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
