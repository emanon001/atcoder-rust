#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        s: Chars
    };

    let mut res = 0_usize;
    for i in 0..n {
        let left_down_count = (0..i).rev().take_while(|j| s[*j] == 'A').count();
        let right_down_count = (i..n - 1).take_while(|j| s[*j] == 'B').count();
        let down_count = left_down_count.max(right_down_count);
        res += 1 + down_count;
    }
    println!("{}", res);
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
