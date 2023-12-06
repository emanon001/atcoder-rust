#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize,
        rectangles: [(usize, usize, usize, usize); n]
    };

    let mut grid = vec![vec![0; 100]; 100];
    for (a, b, c, d) in rectangles {
        for i in a..b {
            for j in c..d {
                grid[i][j] += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..100 {
        for j in 0..100 {
            if grid[i][j] > 0 {
                ans += 1;
            }
        }
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
