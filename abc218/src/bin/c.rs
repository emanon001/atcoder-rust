#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn get(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut l = 0;
    let mut r = 0;
    let mut t = 0;
    let mut b = 0;
    for j in 0..grid.len() {
        let mut ok = false;
        for i in 0..grid.len() {
            if grid[i][j] == '#' {
                ok = true;
                break;
            }
        }
        if ok {
            l = j;
            break;
        }
    }
    for j in (0..grid.len()).rev() {
        let mut ok = false;
        for i in 0..grid.len() {
            if grid[i][j] == '#' {
                ok = true;
                break;
            }
        }
        if ok {
            r = j;
            break;
        }
    }
    for i in 0..grid.len() {
        let mut ok = false;
        for j in 0..grid.len() {
            if grid[i][j] == '#' {
                ok = true;
                break;
            }
        }
        if ok {
            t = i;
            break;
        }
    }
    for i in (0..grid.len()).rev() {
        let mut ok = false;
        for j in 0..grid.len() {
            if grid[i][j] == '#' {
                ok = true;
                break;
            }
        }
        if ok {
            b = i;
            break;
        }
    }
    let mut res = vec![vec!['.'; r - l + 1]; b - t + 1];
    for i in t..=b {
        for j in l..=r {
            res[i - t][j - l] = grid[i][j];
        }
    }
    res
}

fn rotate(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let h = grid.len();
    let w = grid[0].len();
    let mut res = vec![vec!['.'; h]; w];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            res[j][h - i - 1] = grid[i][j];
        }
    }
    res
}

fn solve() {
    input! {
        n: usize,
        grid1: [Chars; n],
        grid2: [Chars; n],
    };

    let grid1 = get(&grid1);
    let grid2 = get(&grid2);
    if grid1 == grid2 {
        println!("Yes");
        return;
    }
    let grid2 = rotate(grid2);
    if grid1 == grid2 {
        println!("Yes");
        return;
    }
    let grid2 = rotate(grid2);
    if grid1 == grid2 {
        println!("Yes");
        return;
    }
    let grid2 = rotate(grid2);
    if grid1 == grid2 {
        println!("Yes");
        return;
    }
    println!("No");
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
