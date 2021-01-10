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
        xyv: [(i64, i64); n]
    };

    let mut res = 0;
    for i in 0..n - 1 {
        for j in i + 1..n {
            let dy = xyv[j].1 - xyv[i].1;
            let dx = xyv[j].0 - xyv[i].0;
            if dy.abs() <= dx.abs() {
                res += 1;
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
