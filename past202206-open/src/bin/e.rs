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
        N: u64
    };

    let mut sum = 0_u64;
    let mut n = 0;
    let mut diff = 0;
    while sum < N {
        n += 1;
        diff = N - sum;
        sum += (n - 1) * 2 + 1;
    }
    let ans = if diff <= n {
        n - (diff - 1)
    } else {
        diff - n + 1
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
