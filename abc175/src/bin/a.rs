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

    let mut res = 0;
    let mut c = 0;
    let mut prev = 'x';
    for ch in s {
        if ch == 'R' {
            if prev == 'R' {
                c += 1;
            } else {
                c = 1;
            }
        } else {
            c = 0;
        };
        if c > res {
            res = c;
        }
        prev = ch;
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
