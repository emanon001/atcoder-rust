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
    };

    let ans = if N.is_odd() {
        let m = (N - 1) / 2;
        let s = "-".repeat(m);
        format!("{}={}", s, s)
    } else {
        let m = (N - 2) / 2;
        let s = "-".repeat(m);
        format!("{}=={}", s, s)
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
