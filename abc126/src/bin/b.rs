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
        s: Chars
    };

    let head2 = s
        .iter()
        .take(2)
        .map(|&ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, x| acc * 10 + x);
    let tail2 = s
        .iter()
        .skip(2)
        .map(|&ch| ch.to_digit(10).unwrap() as usize)
        .fold(0, |acc, x| acc * 10 + x);
    let ok_yymm = (1..=12).contains(&tail2);
    let ok_mmyy = (1..=12).contains(&head2);
    let res = match (ok_yymm, ok_mmyy) {
        (true, true) => "AMBIGUOUS",
        (true, _) => "YYMM",
        (_, true) => "MMYY",
        _ => "NA",
    };
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
