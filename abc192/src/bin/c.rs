#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f1(x: usize) -> usize {
    let mut chars = x.to_string().chars().collect::<Vec<char>>();
    chars.sort();
    chars.reverse();
    chars.into_iter().join("").parse().unwrap()
}

fn f2(x: usize) -> usize {
    let mut chars = x.to_string().chars().collect::<Vec<char>>();
    chars.sort();
    chars.into_iter().join("").parse().unwrap()
}

fn solve() {
    input! {
        n: usize, k: usize
    };
    let mut x = n;
    for _ in 0..k {
        x = f1(x) - f2(x);
    }
    println!("{}", x);
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
