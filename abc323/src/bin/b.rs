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
        n: usize,
        s: [Chars; n]
    };

    let mut win_table = vec![Vec::new(); n];
    for (i, s_i) in s.into_iter().enumerate() {
        let i = i + 1;
        let win_count = s_i.into_iter().filter(|&ch| ch == 'o').count();
        win_table[win_count].push(i);
    }
    let ans = win_table.into_iter().rev().flatten().join(" ");
    println!("{}", ans);
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
