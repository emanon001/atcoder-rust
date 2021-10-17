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

    let mut v = Vec::new();
    for i in 0..s.len() {
        let mut s2 = Vec::new();
        for ii in i..s.len() {
            s2.push(s[ii]);
        }
        for ii in 0..i {
            s2.push(s[ii]);
        }
        v.push(s2);
    }
    v.sort();
    println!("{}", v[0].iter().join(""));
    println!("{}", v[s.len() - 1].iter().join(""));
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
