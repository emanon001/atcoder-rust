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
        mut n: usize
    };

    let mut res: Vec<char> = Vec::new();
    while n > 0 {
        let mut a = n % 26;
        a = if a == 0 { 25 } else { a - 1 };
        res.push((0x61_u8 + a as u8).into());
        n = (n - 1) / 26;
    }
    let res = res.into_iter().rev().collect::<String>();
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
