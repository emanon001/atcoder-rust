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
        x: usize
    };

    if x < 40 {
        println!("{}", 40 - x);
    } else if x < 70 {
        println!("{}", 70 - x);
    } else if x < 90 {
        println!("{}", 90 - x);
    } else {
        println!("expert");
    };
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
