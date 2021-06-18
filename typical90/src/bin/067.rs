#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn calc(digits: Vec<u64>) -> Vec<u64> {
    // 8 -> 10
    let mut ten = digits.into_iter().fold(0_u64, |acc, x| acc * 8_u64 + x);
    // 10 -> 9 (8 -> 5)
    let mut res = VecDeque::new();
    while ten > 0 {
        let d = ten % 9;
        res.push_front(if d == 8 { 5 } else { d });
        ten /= 9;
    }
    if res.is_empty() {
        vec![0]
    } else {
        res.into_iter().collect::<Vec<_>>()
    }
}

fn solve() {
    input! {
        n: Chars,
        k: usize,
    };

    let mut res = n.into_iter().map(|ch| ch.to_digit(10).unwrap() as u64).collect::<Vec<_>>();
    for _ in 0..k {
        res = calc(res);
    }
    println!("{}", res.into_iter().join(""));
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
