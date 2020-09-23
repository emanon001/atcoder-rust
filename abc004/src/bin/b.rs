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
        grid: [[char; 4]; 4]
    };

    let mut res = vec![vec!['x'; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let ch = grid[i][j];
            res[3 - i][3 - j] = ch;
        }
    }
    for i in 0..4 {
        println!("{}", res[i].iter().join(" "));
    }
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
