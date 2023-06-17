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

    let res = if s[0] != s[1] && s[0] != s[2] {
        s[0].to_string()
    } else if s[1] != s[0] && s[1] != s[2] {
        s[1].to_string()
    } else if s[2] != s[0] && s[2] != s[1] {
        s[2].to_string()
    } else {
        "-1".to_string()
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
