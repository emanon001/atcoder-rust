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
        s1: char, s2: char, s3: char,
        t1: char, t2: char, t3: char,
    };

    let mut c = 0;
    if s1 == t1 {
        c += 1;
    }
    if s2 == t2 {
        c += 1;
    }
    if s3 == t3 {
        c += 1;
    }
    let res = if c == 1 { "No" } else { "Yes" };
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
