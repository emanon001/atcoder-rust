#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn rotate_ascii_lowercase_char(ch: char, n: isize) -> char {
    assert!(ch.is_ascii_lowercase());
    let base = 0x61_u8;
    let m = 26_isize;
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
        c: char
    };

    let res = rotate_ascii_lowercase_char(c, 1);
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
