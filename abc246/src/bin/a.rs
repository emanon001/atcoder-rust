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
        x1: isize, y1: isize,
        x2: isize, y2: isize,
        x3: isize, y3: isize,
    };

    let x4 = if x1 == x2 {
        x3
    } else if x1 == x3 {
        x2
    } else {
        x1
    };
    let y4 = if y1 == y2 {
        y3
    } else if y1 == y3 {
        y2
    } else {
        y1
    };
    println!("{} {}", x4, y4);
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
