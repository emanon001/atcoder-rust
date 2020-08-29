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
        t: Chars
    };

    let mut res = 1 << 30;
    for i in 0..=s.len() - t.len() {
        let mut c = 0;
        for j in 0..t.len() {
            if s[i + j] != t[j] {
                c += 1;
            }
        }
        if c < res {
            res = c;
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
