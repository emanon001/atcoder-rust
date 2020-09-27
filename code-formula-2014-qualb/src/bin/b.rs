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
        mut s: Chars
    };

    s.reverse();
    let mut sum_odd = 0;
    let mut sum_even = 0;
    for i in 0..s.len() {
        let d = s[i].to_digit(10).unwrap();
        if (i + 1) % 2 == 1 {
            sum_odd += d;
        } else {
            sum_even += d;
        }
    }
    println!("{} {}", sum_even, sum_odd);
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
