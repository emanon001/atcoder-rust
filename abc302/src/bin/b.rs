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

    let snuke = "snuke".chars().collect::<Vec<_>>();

    for i in 0..h {
        for j in 0..w {
            let dirs: Vec<(isize, isize)> = vec![
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ];

            'dirs: for (di, dj) in dirs {
                let mut ans = Vec::new();
                for k in 0..5 {
                    let new_i = i as isize + di * k as isize;
                    let new_j = j as isize + dj * k as isize;
                    if new_i < 0 || new_i >= h as isize || new_j < 0 || new_j >= w as isize {
                        continue 'dirs;
                    }
                    if snuke[k] != grid[new_i as usize][new_j as usize] {
                        continue 'dirs;
                    }
                    ans.push((new_i + 1, new_j + 1));
                }
                println!(
                    "{}",
                    ans.iter().map(|(i, j)| format!("{} {}", i, j)).join("\n")
                );
            }
        }
    }
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
