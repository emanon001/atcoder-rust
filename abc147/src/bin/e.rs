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
        grid_a: [[usize; w]; h],
        grid_b: [[usize; w]; h],
    };

    let max_v = 80_usize * 160;
    let mut used = vec![vec![vec![false; max_v * 2 + 1]; w]; h];
    used[0][0][max_v] = true;
    let mut res = std::isize::MAX;
    for i in 0..h {
        for j in 0..w {
            let a = grid_a[i][j];
            let b = grid_b[i][j];
            // ex: max_v: 2
            // -2, -1, 0, 1, 2
            for k in 0..max_v * 2 + 1 {
                if used[i][j][k] {
                    if i == h - 1 && j == w - 1 {
                        let v = ((k + a - b) as isize - max_v as isize).abs();
                        if v < res {
                            res = v;
                        }
                        let v = ((k + b - a) as isize - max_v as isize).abs();
                        if v < res {
                            res = v;
                        }
                    }
                    if i + 1 < h {
                        let new_i = i + 1;
                        let new_j = j;
                        used[new_i][new_j][k + a - b] = true;
                        used[new_i][new_j][k + b - a] = true;
                    }
                    if j + 1 < w {
                        let new_i = i;
                        let new_j = j + 1;
                        used[new_i][new_j][k + a - b] = true;
                        used[new_i][new_j][k + b - a] = true;
                    }
                }
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
