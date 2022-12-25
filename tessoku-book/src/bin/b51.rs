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
        s: Chars,
    };

    let mut stack = Vec::new();
    for (i, c) in s.into_iter().enumerate() {
        if c == '(' {
            stack.push(i + 1);
        } else {
            let s = stack.pop().unwrap();
            let e = i + 1;
            println!("{} {}", s, e);
        }
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
