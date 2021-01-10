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
        a: Chars, b: Chars
    };

    let a: u32 = a.into_iter().map(|ch| ch.to_digit(10).unwrap()).sum();
    let b: u32 = b.into_iter().map(|ch| ch.to_digit(10).unwrap()).sum();
    let res = a.max(b);
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
