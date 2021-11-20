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
        n: usize, k: i64,
        pv: [[usize; 3]; n]
    };

    let mut points = vec![0_i64; 900 + 1];
    for i in 0..n {
        let p = pv[i][0] + pv[i][1] + pv[i][2];
        points[p] += 1;
    }
    for i in 0..n {
        let p = pv[i][0] + pv[i][1] + pv[i][2];
        let mut sum = 0;
        for j in (p + 300 + 1)..=900 {
            sum += points[j];
        }
        sum += 1;
        let res = if sum <= k { "Yes" } else { "No" };
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
