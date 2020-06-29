#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn shift_ascii_uppercase_char(ch: char, n: usize) -> char {
    assert!(ch.is_ascii_uppercase());
    let base = 0x41_u8;
    let m = 26;
    let offset = (ch as u8 - base + (n % m) as u8) % m as u8;
    char::from(base + offset)
}

fn solve() {
    input! {
        n: usize,
        s: Chars
    };

    let res = s
        .into_iter()
        .map(|ch| shift_ascii_uppercase_char(ch, n))
        .join("");
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
