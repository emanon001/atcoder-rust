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

    let mut pv = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == 'o' {
                pv.push((i as isize, j as isize));
            }
        }
    }
    let p1 = pv[0];
    let p2 = pv[1];
    let res = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs();
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
