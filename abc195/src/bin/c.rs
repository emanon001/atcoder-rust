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
        n: Chars
    };

    let digits = n
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .collect::<Vec<_>>();
    let len = digits.len();
    let mut res = 0_u64;
    // same
    if len > 3 {
        let c = (len - 1) as u64 / 3;
        res += c * (digits[1..].into_iter().fold(0_u64, |acc, x| acc * 10 + x) + 1);
    }
    // lower
    for i in 0..len {
        if len - i <= 3 {
            continue;
        }
        let cur_d = digits[i] as usize;
        let max_d = if i == 0 { if cur_d > 0 { cur_d - 1 } else { 0 } } else { 9 };
        for _ in 1..=max_d {
            let c = (len - i - 1) as u64 / 3;
            res += c * 10_u64.pow((len - i - 1) as u32);
        }
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
