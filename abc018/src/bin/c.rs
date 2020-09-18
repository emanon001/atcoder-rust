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
        r: usize, c: usize, k: usize,
        grid: [Chars; r]
    };

    let mut cusum = vec![vec![0; c]; r];
    for i in 0..r {
        let mut count = 0;
        for j in 0..c {
            if grid[i][j] == 'x' {
                count += 1;
            }
            cusum[i][j] = count;
        }
    }
    let mut res = 0;
    for i in 0..r {
        for j in 0..c {
            if i + 1 < k || i + k - 1 >= r || j + 1 < k || j + k - 1 >= c {
                continue;
            }
            let mut count = 0;
            for l in 0..k {
                let ci = i + 1 - k + l;
                count += cusum[ci][c - 1];
                count -= if j - l == 0 { 0 } else { cusum[ci][j - l - 1] };
                count -= cusum[ci][c - 1] - cusum[ci][j + l];
            }
            for l in 0..k - 1 {
                let ci = i + k - 1 - l;
                count += cusum[ci][c - 1];
                count -= if j - l == 0 { 0 } else { cusum[ci][j - l - 1] };
                count -= cusum[ci][c - 1] - cusum[ci][j + l];
            }
            if count == 0 {
                res += 1;
            }
        }
    }
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
