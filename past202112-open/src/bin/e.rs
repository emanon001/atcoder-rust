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
        nv: Chars,
    };

    let nv = nv
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    let mut res = 0_i32;
    let mut prev_n = -1;
    let mut prev_hand = "none";
    for (i, n) in nv.into_iter().enumerate() {
        let hand = if [1, 2, 3, 4, 5].contains(&n) {
            "left"
        } else {
            "right"
        };
        res += if i == 0 {
            500
        } else if n == prev_n {
            301
        } else if hand == prev_hand {
            210
        } else {
            100
        };
        prev_n = n;
        prev_hand = hand;
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
