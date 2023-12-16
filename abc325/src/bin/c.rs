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
        h: usize, w: usize,
        grid: [Chars; h],
    };

    let mut ans = 0;
    let mut visited = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '.' {
                continue;
            }
            if visited[i][j] {
                continue;
            }
            ans += 1;
            traverse(i, j, h, w, &grid, &mut visited);
        }
    }
    println!("{}", ans);
}

fn traverse(i: usize, j: usize, h: usize, w: usize, grid: &[Vec<char>], visited: &mut [Vec<bool>]) {
    visited[i][j] = true;
    for di in -1..=1 {
        for dj in -1..=1 {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if grid[ni][nj] == '.' {
                continue;
            }
            if visited[ni][nj] {
                continue;
            }
            traverse(ni, nj, h, w, grid, visited);
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
