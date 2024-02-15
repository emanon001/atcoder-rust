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
        X: String
    };

    let counts = X.chars().counts();
    let power_of_10 =
        *counts.get(&'1').unwrap_or(&0) == 1 && *counts.get(&'0').unwrap_or(&0) == X.len() - 1;
    let ans = if power_of_10 { X.len() - 1 } else { X.len() };
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
