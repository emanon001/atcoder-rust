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
        X: String,
    };

    let x = X.split(".").collect_vec();
    let a = x[0].parse::<i32>().unwrap();
    let b = x[1].parse::<i32>().unwrap();
    let c = a * 10 + b;
    let ans = if c >= 380 {
        1
    } else if c >= 375 {
        2
    } else {
        3
    };
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
