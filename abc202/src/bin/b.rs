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
        mut s: Chars
    };

    s.reverse();
    for i in 0..s.len() {
        if s[i] == '6' {
            s[i] = '9';
        } else if s[i] == '9' {
            s[i] = '6';
        }
    }
    println!("{}", s.iter().join(""));
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
