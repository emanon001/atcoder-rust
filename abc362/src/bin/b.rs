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
        p1: (isize, isize),
        p2: (isize, isize),
        p3: (isize, isize),
    };

    let ans = if is_right_triangle(p1, p2, p3) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

fn is_right_triangle(p1: (isize, isize), p2: (isize, isize), p3: (isize, isize)) -> bool {
    // p1, p2間の距離
    let d1 = (p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2);
    // p2, p3間の距離
    let d2 = (p2.0 - p3.0).pow(2) + (p2.1 - p3.1).pow(2);
    // p3, p1間の距離
    let d3 = (p3.0 - p1.0).pow(2) + (p3.1 - p1.1).pow(2);
    d1 + d2 == d3 || d1 + d3 == d2 || d2 + d3 == d1
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
