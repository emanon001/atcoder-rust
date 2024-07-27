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
        N: usize, K: i64,
        A: [i64; N],
        B: [i64; N],
    };

    let diff = A
        .into_iter()
        .zip(B.into_iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<i64>();
    let ans = if diff <= K && (K - diff).is_even() {
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
