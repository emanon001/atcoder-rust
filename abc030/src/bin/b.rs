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
        n: usize, m: usize
    };

    let n = n % 12;
    let n_pos = 360_f64 / 720.0 * (n * 60 + m) as f64;
    let m_pos = 360_f64 / 60.0 * m as f64;
    let res = (n_pos - m_pos).abs().min(360.0 - (n_pos - m_pos).abs());
    println!("{}", res);
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
