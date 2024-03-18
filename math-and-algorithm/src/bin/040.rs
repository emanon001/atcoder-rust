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
        N: usize,
        A: [i64; N - 1],
        M: usize,
        B: [Usize1; M],
    };

    let mut cusum = vec![0; N];
    for i in 0..N - 1 {
        cusum[i + 1] = cusum[i] + A[i];
    }
    let ans = B
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| {
            let i = a.min(b);
            let j = a.max(b);
            cusum[j] - cusum[i]
        })
        .sum::<i64>();
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
