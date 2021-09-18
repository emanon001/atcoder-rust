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
        s1: String,
        s2: String,
        s3: String,
        t: Chars
    };

    let mut res = Vec::new();
    for ch in t {
        if ch == '1' {
            res.push(&s1);
        } else if ch == '2' {
            res.push(&s2);
        } else {
            res.push(&s3);
        }
    }
    println!("{}", res.iter().join(""));
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
