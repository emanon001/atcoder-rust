#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn is_chokugo(s: &str) -> bool {
    if s.is_empty() {
        return true;
    }
    if s.starts_with("ch") {
        is_chokugo(&s[2..])
    } else if s.starts_with("o") {
        is_chokugo(&s[1..])
    } else if s.starts_with("k") {
        is_chokugo(&s[1..])
    } else if s.starts_with("u") {
        is_chokugo(&s[1..])
    } else {
        false
    }
}

fn solve() {
    input! {
        x: String
    };

    let res = if is_chokugo(&x) { "YES" } else { "NO" };
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
