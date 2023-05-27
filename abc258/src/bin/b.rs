#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// 上下左右 + 斜め (i, j)
pub const ALL_DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn solve() {
    input! {
        n: usize,
        grid: [Chars; n]
    };

    let mut candidates: Vec<Vec<usize>> = Vec::new();
    for i in 0..n {
        for j in 0..n {
            for dir in &ALL_DIRS {
                let mut path = Vec::new();
                let mut pos = (i, j);
                for _ in 0..n {
                    let v = grid[pos.0][pos.1].to_digit(10).unwrap() as usize;
                    path.push(v);
                    let mut new_i = (pos.0 as isize) + dir.0;
                    new_i = if new_i < 0 {
                        n as isize + new_i
                    } else {
                        new_i % n as isize
                    };
                    let mut new_j = (pos.1 as isize) + dir.1;
                    new_j = if new_j < 0 {
                        n as isize + new_j
                    } else {
                        new_j % n as isize
                    };
                    pos = (new_i as usize, new_j as usize);
                }
                candidates.push(path.clone());
            }
        }
    }
    candidates.sort_by(|a, b| b.cmp(a));
    println!("{}", candidates[0].iter().join(""));
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
