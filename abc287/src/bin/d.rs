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
        s: Chars,
        t: Chars,
    };

    let mut forward = vec![false; t.len() + 1];
    forward[0] = true;
    for i in 0..t.len() {
        forward[i + 1] = forward[i] && (s[i] == '?' || t[i] == '?' || s[i] == t[i]);
    }
    let mut backward = vec![false; t.len() + 1];
    backward[0] = true;
    for i in 0..t.len() {
        let si = s.len() - 1 - i;
        let ti = t.len() - 1 - i;
        backward[i + 1] = backward[i] && (s[si] == '?' || t[ti] == '?' || s[si] == t[ti]);
    }
    // eprintln!("{:?}", forward);
    // eprintln!("{:?}", backward);
    for i in 0..=t.len() {
        let res = if forward[i] && backward[t.len() - i] {
            "Yes"
        } else {
            "No"
        };
        println!("{}", res);
    }
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
