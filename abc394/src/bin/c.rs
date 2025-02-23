#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
use regex::Regex;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        S: String,
    };

    // W+A を AC+ に変換
    let re = Regex::new(r"W+A").unwrap();
    let ans = re
        .replace_all(&S, |caps: &regex::Captures| {
            let len = caps[0].len();
            format!("A{}", "C".repeat(len - 1))
        })
        .to_string();
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
