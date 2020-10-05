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

fn is_match(s: &[char], t: &[char]) -> bool {
    if s.len() < t.len() {
        return false;
    }
    (0..=s.len() - t.len()).any(|i| (0..t.len()).all(|j| s[i + j] == t[j] || t[j] == '.'))
}

fn solve() {
    input! {
        s: Chars
    };

    let mut chars = make_ascii_lowercase_chars();
    chars.push('.');
    let mut res = 0;
    for &a in &chars {
        let mut t = vec![a];
        if is_match(&s, &t) {
            res += 1;
        }
        for &b in &chars {
            t.push(b);
            if is_match(&s, &t) {
                res += 1;
            }
            for &c in &chars {
                t.push(c);
                if is_match(&s, &t) {
                    res += 1;
                }
                t.pop();
            }
            t.pop();
        }
    }
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
