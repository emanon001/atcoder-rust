#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(a: usize, b: usize, c: usize, dp: &mut [Vec<Vec<Option<f64>>>]) -> f64 {
    if let Some(x) = dp[a][b][c] {
        return x;
    }
    if a == 100 || b == 100 || c == 100 {
        return 0.0;
    }
    let count = (a + b + c) as f64;
    let mut res = 0_f64;
    // aの場合
    res += (f(a + 1, b, c, dp) + 1.0) * (a as f64 / count as f64);
    // bの場合
    res += (f(a, b + 1, c, dp) + 1.0) * (b as f64 / count as f64);
    // cの場合
    res += (f(a, b, c + 1, dp) + 1.0) * (c as f64 / count as f64);
    dp[a][b][c] = Some(res);
    return res;
}

fn solve() {
    input! {
        a: usize, b: usize, c: usize,
    };
    // dp[x][y][z] = aがx個、bがy個、cがz個の確率
    let mut dp = vec![vec![vec![None; 100 + 1]; 100 + 1]; 100 + 1];
    let res = f(a, b, c, &mut dp);
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
