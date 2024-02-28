#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        S: [String; N],
    };

    let max_len = S.iter().map(|s| s.len()).max().unwrap();
    let ans = S
        .into_iter()
        .sorted_by_key(|s| {
            let c = s.chars().take_while(|&ch| ch == '0').count();
            (
                format!("{:0>width$}", s, width = max_len),
                std::cmp::Reverse(c),
            )
        })
        .join("\n");
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
