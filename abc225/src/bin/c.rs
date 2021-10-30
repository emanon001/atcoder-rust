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
        n: usize, m: usize,
        grid: [[i64; m]; n]
    };

    // 枠外
    for i in 0..n {
        let mut pos = grid[i][0] % 7;
        if pos == 0 {
            pos = 7;
        }
        if pos + m as i64 - 1 > 7 {
            println!("No");
            return;
        }
    }

    // 先頭の位置
    let mut pos_set = HashSet::new();
    for i in 0..n {
        let pos = grid[i][0] % 7;
        pos_set.insert(pos);
    }
    if pos_set.len() > 1 {
        println!("No");
        return;
    }

    // 連続しているか
    for i in 0..n {
        for j in 1..m {
            if grid[i][j] != grid[i][j - 1] + 1 {
                println!("No");
                return;
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
