#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn color_to_score(c: char) -> usize {
    match c {
        'W' => 0,
        'B' => 1,
        'R' => 2,
        _ => unreachable!(),
    }
}

fn solve() {
    input! {
        _: usize, c: char,
        av: Chars
    };

    let sum = av.into_iter().fold(0, |acc, c| acc + color_to_score(c));
    let res = if sum % 3 == color_to_score(c) {
        "Yes"
    } else {
        "No"
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
