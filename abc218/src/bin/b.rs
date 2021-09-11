#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn make_ascii_lowercase_chars() -> Vec<char> {
    let base = 0x61;
    make_chars(base..base + 26)
}
pub fn make_ascii_uppercase_chars() -> Vec<char> {
    let base = 0x41;
    make_chars(base..base + 26)
}
fn make_chars(range: std::ops::Range<u8>) -> Vec<char> {
    range.into_iter().map(char::from).collect::<Vec<_>>()
}
pub fn rotate_ascii_lowercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_lowercase());
    rotate_char(ch, n, 0x61, 26)
}
pub fn rotate_ascii_uppercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_uppercase());
    rotate_char(ch, n, 0x41, 26)
}
fn rotate_char(ch: char, n: isize, base: u8, m: u8) -> char {
    let m = m as isize;
    let ch_pos = ch as u8 - base;
    let offset = if n >= 0 {
        (ch_pos as isize + n) % m
    } else {
        (m - (ch_pos as isize - n.abs()).abs() % m) % m
    } as u8;
    char::from(base + offset)
}

fn solve() {
    input! {
        pv: [Usize1; 26]
    };
    let mut res = Vec::new();
    for p in pv {
        res.push((p as u8 + 0x61) as char);
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
