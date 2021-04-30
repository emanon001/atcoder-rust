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
        h: usize, w: usize, x: Usize1, y: Usize1,
        grid: [Chars; h]
    };

    let mut res = 1;
    // left
    for i in (0..y).rev() {
        if grid[x][i] == '#' {
            break;
        }
        res += 1;
    }
    // right
    for i in y + 1..w {
        if grid[x][i] == '#' {
            break;
        }
        res += 1;
    }
    // top
    for i in (0..x).rev() {
        if grid[i][y] == '#' {
            break;
        }
        res += 1;
    }
    // bottom
    for i in x + 1..h {
        if grid[i][y] == '#' {
            break;
        }
        res += 1;
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
