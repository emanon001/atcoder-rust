#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn solve() {
    input! {
        h: usize, w: usize, n: usize,
        grid: [[Usize1; w]; h],
        cv: [usize; n]
    };

    for i in 0..h {
        for j in 0..w {
            let s = grid[i][j];
            let c = cv[s];
            for (di, dj) in &UDLR_DIRS {
                let i2 = i as isize + di;
                let j2 = j as isize + dj;
                if (0..h as isize).contains(&i2) && (0..w as isize).contains(&j2) {
                    let i2 = i2 as usize;
                    let j2 = j2 as usize;
                    let s2 = grid[i2][j2];
                    let c2 = cv[s2];
                    if s != s2 && c == c2 {
                        println!("No");
                        return;
                    }
                }
            }
        }
    }
    println!("Yes");
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
