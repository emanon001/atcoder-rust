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
        n: usize, a: usize, b: usize,
    };

    let mut res = vec![vec![' '; n * b]; n * a];
    for i in 0..n {
        for j in 0..n {
            let ch = if i.is_even() && j.is_even() || i.is_odd() && j.is_odd() {
                '.'
            } else {
                '#'
            };
            for i_a in 0..a {
                for j_b in 0..b {
                    res[i * a + i_a][j * b + j_b] = ch;
                }
            }
        }
    }
    println!(
        "{}",
        res.into_iter()
            .map(|row| row.into_iter().join(""))
            .join("\n")
    );
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
