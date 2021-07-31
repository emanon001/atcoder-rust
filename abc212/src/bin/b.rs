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
        s: Chars
    };

    let s = s.into_iter().map(|ch| ch.to_digit(10).unwrap()).collect::<Vec<_>>();
    if s[0] == s[1] && s[1] == s[2] && s[2] == s[3] {
        println!("Weak");
        return;
    }
    let is_strong = (s[0] + 1) % 10 != s[1] || (s[1] + 1) % 10 != s[2] || (s[2] + 1) % 10 != s[3];
    let res = if is_strong { "Strong" } else { "Weak" };
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
