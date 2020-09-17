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
        a: isize, b: isize, c: isize
    };

    let mut v = vec![a, b, c];
    v.sort_by_key(|x| -x);
    for x in vec![a, b, c] {
        println!("{}", v.iter().position(|y| y == &x).unwrap() + 1);
    }
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
