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
        s: Chars
    };

    let mut v = Vec::new();
    for ch in s {
        v.push(ch);
        if v.len() >= 3 && v[v.len() - 3..] == ['f', 'o', 'x'] {
            v.pop(); v.pop(); v.pop();
        }
    }
    println!("{}", v.len());
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
