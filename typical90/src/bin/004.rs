#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        A: [[i64; W]; H],
    };

    let mut vertical_sum = vec![0; W];
    for j in 0..W {
        for i in 0..H {
            vertical_sum[j] += A[i][j];
        }
    }
    let mut horizontal_sum = vec![0; H];
    for i in 0..H {
        for j in 0..W {
            horizontal_sum[i] += A[i][j];
        }
    }

    let mut ans = vec![vec![0; W]; H];
    for i in 0..H {
        for j in 0..W {
            ans[i][j] = vertical_sum[j] + horizontal_sum[i] - A[i][j];
        }
    }
    println!("{}", ans.iter().map(|v| v.iter().join(" ")).join("\n"));
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
