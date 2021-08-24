#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize, k: usize,
        abv: [(Usize1, Usize1); n]
    };

    let grid_size = 5000;
    let mut grid = vec![vec![0_i32; grid_size]; grid_size];
    for (a, b) in abv {
        grid[a][b] += 1;
    }
    let mut cusum = vec![vec![0; grid_size + 1]; grid_size + 1];
    for i in 0..grid_size {
        for j in 0..grid_size {
            cusum[i + 1][j + 1] = cusum[i + 1][j] + cusum[i][j + 1] - cusum[i][j] + grid[i][j];
        }
    }
    let mut res = 0_i32;
    for i in 0..grid_size {
        for j in 0..grid_size {
            let ii = (i + k + 1).min(grid_size);
            let jj = (j + k + 1).min(grid_size);
            let c = cusum[ii][jj] - cusum[ii][j] - cusum[i][jj] + cusum[i][j];
            chmax!(res, c);
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
