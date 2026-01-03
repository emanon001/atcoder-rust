#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn make_ascii_lowercase_chars() -> Vec<char> {
    let base = 0x61;
    make_chars(base..base + 26)
}
fn make_chars(range: std::ops::Range<u8>) -> Vec<char> {
    range.into_iter().map(char::from).collect::<Vec<_>>()
}

#[allow(non_snake_case)]
fn main() {
    input_interactive! {
        S: Chars,
    };

    let mut set = make_ascii_lowercase_chars()
        .into_iter()
        .collect::<HashSet<_>>();
    for ch in S {
        set.remove(&ch);
    }
    let ans = set.into_iter().next().unwrap();
    println!("{}", ans);
}
