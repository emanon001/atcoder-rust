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
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut res = Vec::new();
    for j in 0..w {
        let mut count = 0;
        for i in 0..h {
            if grid[i][j] == '#' {
                count += 1;
            }
        }
        res.push(count);
    }
    println!("{}", res.iter().join(" "));
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
