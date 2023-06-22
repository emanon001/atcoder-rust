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
        n: usize,
        grid: [Chars; n]
    };

    for i in 0..n {
        for j in i + 1..n {
            let ch1 = grid[i][j];
            let ch2 = grid[j][i];
            let valid = matches!((ch1, ch2), ('W', 'L') | ('L', 'W') | ('D', 'D'));
            if !valid {
                println!("incorrect");
                return;
            }
        }
    }
    println!("correct");
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
