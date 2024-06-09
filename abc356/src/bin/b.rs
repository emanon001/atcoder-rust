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
        N: usize, M: usize,
        A: [usize; M],
        X: [[usize; M]; N],
    };

    let mut sum = vec![0; M];
    for i in 0..N {
        for j in 0..M {
            sum[j] += X[i][j];
        }
    }
    let ans = if sum.into_iter().enumerate().all(|(i, x)| x >= A[i]) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
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
