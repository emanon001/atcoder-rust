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
        N: usize, Q: usize,
        A: [i64; N],
        LR: [(usize, usize); Q],
    };

    let mut cusum = vec![0; N + 1];
    for i in 1..=N {
        cusum[i] = cusum[i - 1] + A[i - 1];
    }
    for (l, r) in LR {
        let ans = cusum[r] - cusum[l - 1];
        println!("{}", ans);
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
