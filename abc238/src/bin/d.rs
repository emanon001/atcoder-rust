#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input! {
        T: usize,
        TESTS: [(u64, u64); T]
    };

    for (a, s) in TESTS {
        let a2 = a * 2;
        let ans = if s >= a2 && (((s - a2) & a) == 0) {
            "Yes"
        } else {
            "No"
        };
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
