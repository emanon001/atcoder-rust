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
        N: usize,
        XY: [(f64, f64); N],
    };

    let mut ans = 0_f64;
    let mut point = (0_f64, 0_f64);
    for (x, y) in XY.into_iter().chain(std::iter::once((0_f64, 0_f64))) {
        ans += ((point.0 - x).powi(2) + (point.1 - y).powi(2)).sqrt();
        point = (x, y);
    }
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
