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

    let mut diff = HashSet::new();
    for i in 0..s.len() {
        let a = s[i] as u8 - 0x61;
        let b = t[i] as u8 - 0x61;
        let d = (b + 26 - a) % 26;
        diff.insert(d);
    }
    let res = if diff.len() == 1 { "Yes" } else { "No" };
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
