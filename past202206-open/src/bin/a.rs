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
        X: i64, A: i64, B: i64, C: i64
    };

    let usagi = (C * A + X) * B;
    let kame = X * A;
    let ans = match usagi.cmp(&kame) {
        std::cmp::Ordering::Less => "Hare",
        std::cmp::Ordering::Equal => "Tie",
        std::cmp::Ordering::Greater => "Tortoise",
    };
    println!("{ans}");
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
