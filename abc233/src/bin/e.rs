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
        x: Chars
    };

    let xv = x
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as i64)
        .collect::<Vec<_>>();
    let mut cusum = vec![0; xv.len() + 1];
    let mut cur = 0_i64;
    for i in 0..xv.len() {
        cur += xv[i];
        cusum[i] = cur;
    }
    let mut res = Vec::new();
    let mut add = 0_i64;
    for i in (0..xv.len()).rev() {
        let a = cusum[i] + add;
        let b = a % 10;
        add = a / 10;
        res.push(b);
    }
    if add > 0 {
        res.push(add);
    }
    res.reverse();
    println!("{}", res.into_iter().join(""));
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
