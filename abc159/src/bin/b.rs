#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn is_palindrome(s: &[char]) -> bool {
    let rev = s.into_iter().copied().rev().collect::<Vec<_>>();
    s == &rev[..]
}

fn solve() {
    input! {
        s: Chars
    };

    let len = s.len();
    let mid = len / 2;
    let is_ok = is_palindrome(&s) && is_palindrome(&s[0..mid]) && is_palindrome(&s[mid + 1..]);
    let res = if is_ok { "Yes" } else { "No" };
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
