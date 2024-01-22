#[allow(unused_imports)]
use itertools::Itertools;
use ndarray::Order;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        XY: [(usize, usize); N]
    };

    let mut x_sum = 0;
    let mut y_sum = 0;
    for (x, y) in XY {
        x_sum += x;
        y_sum += y;
    }
    let ans = match x_sum.cmp(&y_sum) {
        Ordering::Equal => "Draw",
        Ordering::Greater => "Takahashi",
        Ordering::Less => "Aoki",
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
