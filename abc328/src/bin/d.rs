#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        s: Chars
    };

    let mut ans = Vec::new();
    for ch in s {
        if ch == 'C' && ans.len() >= 2 && ans[ans.len() - 1] == 'B' && ans[ans.len() - 2] == 'A' {
            ans.pop();
            ans.pop();
        } else {
            ans.push(ch);
        }
    }
    println!("{}", ans.into_iter().join(""));
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
