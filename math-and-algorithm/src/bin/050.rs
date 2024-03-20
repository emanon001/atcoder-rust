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
        a: u64, b: u64,
    };

    let ans = pow(a, b);
    println!("{}", ans);
}

fn pow(a: u64, b: u64) -> u64 {
    if b == 0 {
        return 1;
    }
    let mut res = pow(a, b >> 1);
    res = (res * res) % 1000000007;
    if b & 1 == 1 {
        res = (a * res) % 1000000007;
    }
    res
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
