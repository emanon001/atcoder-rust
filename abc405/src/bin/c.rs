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
fn main() {
    input_interactive! {
        N: usize,
        A: [u64; N],
    };

    let mut cusum = vec![0; N + 1];
    for i in 0..N {
        cusum[i + 1] = cusum[i] + A[i]
    }
    let mut ans = 0_u64;
    for i in 0..N {
        ans += A[i] * (cusum[N] - cusum[i + 1])
    }
    println!("{}", ans);
}
