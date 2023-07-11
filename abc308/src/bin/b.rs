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
        n: usize, m: usize,
        cv: [String; n],
        dv: [String; m],
        pv: [i64; m + 1],
    };

    let mut res = 0_i64;
    for c in cv {
        let j = dv
            .iter()
            .position(|c2| &c == c2)
            .map(|j| j + 1)
            .unwrap_or(0);
        res += pv[j];
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
