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
        M: usize, D: usize,
        y: usize, m: usize, d: usize,
    };

    let is_curry_up_m = d == D;
    let next_d = if is_curry_up_m { 1 } else { d + 1 };

    let is_curry_up_y = is_curry_up_m && m == M;
    let next_m = if is_curry_up_y {
        1
    } else if is_curry_up_m {
        m + 1
    } else {
        m
    };

    let next_y = if is_curry_up_y { y + 1 } else { y };

    println!("{} {} {}", next_y, next_m, next_d);
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
