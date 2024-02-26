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
        S: Chars
    };

    let s = S
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect_vec();
    let check_digit = s[s.len() - 1];
    let s = &s[..s.len() - 1];
    let a = s
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, &x)| x)
        .sum::<u32>()
        * 3;
    let b = s
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, &x)| x)
        .sum::<u32>();
    let c = (a + b) % 10;
    let ans = if c == check_digit { "Yes" } else { "No" };
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
