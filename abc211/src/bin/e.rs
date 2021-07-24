#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(pos: (usize, usize), count: i64, grid: &Vec<Vec<char>>, used: u64, k: i64, res: &mut HashSet<u64>) {
    let h = grid.len();
    let w = h;
    if count == k {
        res.insert(used.clone());
        return;
    }
    let (i, j) = pos;
    // 上
    if i > 0 && grid[i - 1][j] == '.' {
        let new_pos = (i - 1, j);
        let bit_pos = new_pos.0 * w + new_pos.1;
        let new_bits = 1 << bit_pos;
        if (used >> bit_pos) & 1 == 0 {
            dfs(new_pos, count + 1, grid, used | new_bits, k, res);
        }
    }
    // 下
    if i + 1 < h && grid[i + 1][j] == '.' {
        let new_pos = (i + 1, j);
        let bit_pos = new_pos.0 * w + new_pos.1;
        let new_bits = 1 << bit_pos;
        if (used >> bit_pos) & 1 == 0 {
            dfs(new_pos, count + 1, grid, used | new_bits, k, res);
        }
    }
    // 左
    if j > 0 && grid[i][j - 1] == '.' {
        let new_pos = (i, j - 1);
        let bit_pos = new_pos.0 * w + new_pos.1;
        let new_bits = 1 << bit_pos;
        if (used >> bit_pos) & 1 == 0 {
            dfs(new_pos, count + 1, grid, used | new_bits, k, res);
        }
    }
    // 右
    if j + 1 < w && grid[i][j + 1] == '.' {
        let new_pos = (i, j + 1);
        let bit_pos = new_pos.0 * w + new_pos.1;
        let new_bits = 1 << bit_pos;
        if (used >> bit_pos) & 1 == 0 {
            dfs(new_pos, count + 1, grid, used | new_bits, k, res);
        }
    }
}

fn solve() {
    input! {
        n: usize,
        k: usize,
        grid: [Chars; n]
    };

    let mut res = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            if grid[i][j] == '.' {
                let used = 1_u64 << (i * n + j);
                dfs((i, j), 1, &grid, used, k as i64, &mut res);
            }
        }
    }
    println!("{}", res.len());
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
