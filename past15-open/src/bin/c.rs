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
        X: isize, Y: isize, R: isize, N: usize,
    };

    let mut ans = vec![vec!['.'; 2 * N + 1]; 2 * N + 1];
    for i in 0..2 * N + 1 {
        for j in 0..2 * N + 1 {
            let x = i as isize - N as isize;
            let y = j as isize - N as isize;
            if (X - x).pow(2) + (Y - y).pow(2) <= R.pow(2) {
                ans[i][j] = '#';
            }
        }
    }
    println!("{}", ans.iter().map(|row| row.iter().join(" ")).join("\n"));
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
