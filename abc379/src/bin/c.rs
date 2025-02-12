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
        X: [i128; M],
        mut A: [i128; M],
    };

    let sum = A.iter().sum::<i128>();
    if sum != N as i128 {
        println!("-1");
        return;
    }

    let mut xa = X.into_iter().zip(A.into_iter()).collect::<Vec<_>>();
    xa.sort_by_key(|&(x, _)| x);
    let mut ans = 0;
    let mut j = N as i128;
    for (i, a) in xa.into_iter().rev() {
        let count = (j - i + 1).min(a);
        ans += ((j - i + 1) * (j - i) / 2) - ((j - i - count + 1) * (j - i - count) / 2);
        j -= count;
    }
    if j > 0 {
        println!("-1");
        return;
    }
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
