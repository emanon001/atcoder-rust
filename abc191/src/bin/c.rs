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

    let mut blocks = Vec::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                blocks.push((i, j));
            }
        }
    }
    let mut res = 0;
    let dirs = vec![(-1, 0, false), (1, 0, false), (0, -1, true), (0, 1, true)];
    for (i, j) in blocks {
        for &(di, dj, horizontal) in &dirs {
            let new_i = (i as isize + di) as usize;
            let new_j = (j as isize + dj) as usize;
            if grid[new_i][new_j] == '#' {
                continue;
            }
            if horizontal {
                if grid[i + 1][j] == '#' && grid[i + 1][new_j] == '.' {
                    continue;
                }
                res += 1;
            } else {
                if grid[i][j - 1] == '#' && grid[new_i][j - 1] == '.' {
                    continue;
                }
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
