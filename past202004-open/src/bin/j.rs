#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn parse_ascii_chars(i: usize, chars: &[char]) -> (usize, Vec<char>) {
    let mut j = i;
    let mut s = Vec::new();
    while j < chars.len() && chars[j].is_ascii_alphabetic() {
        s.push(chars[j]);
        j += 1;
    }
    (j, s)
}

fn parse_parentheses(i: usize, chars: &[char]) -> (usize, Vec<char>) {
    let mut j = i + 1;
    let mut s = Vec::new();
    while j < chars.len() && chars[j] != ')' {
        let x = parse_expr(j, chars);
        s.extend(x.1);
        j = x.0;
    }
    j += 1;
    let mut rs = s.clone();
    rs.reverse();
    s.append(&mut rs);
    (j, s)
}

// "abc" | (<expr>[, <expr>, ...])
fn parse_expr(i: usize, chars: &[char]) -> (usize, Vec<char>) {
    if chars[i] == '(' {
        parse_parentheses(i, chars)
    } else {
        parse_ascii_chars(i, chars)
    }
}

fn parse(i: usize, chars: &[char]) -> (usize, Vec<char>) {
    let mut j = i;
    let mut s = Vec::new();
    while j < chars.len() {
        let mut expr = parse_expr(j, chars);
        j = expr.0;
        s.append(&mut expr.1);
    }
    (j, s)
}

fn solve() {
    input! {
        s: Chars
    };

    let res = parse(0, &s).1.into_iter().join("");
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
