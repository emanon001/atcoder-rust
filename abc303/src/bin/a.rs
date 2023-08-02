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
        _: usize,
        s: String,
        t: String,
    };

    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();

    let res = if s.into_iter().zip(t.into_iter()).all(|(x, y)| {
        x == y
            || (x == '1' && y == 'l')
            || (y == '1' && x == 'l')
            || (x == '0' && y == 'o')
            || (y == '0' && x == 'o')
    }) {
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
