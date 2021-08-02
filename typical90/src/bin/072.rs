#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn dfs(pos: (usize, usize), k: isize, used: &mut HashSet<(usize, usize)>, start: (usize, usize), size: (usize, usize), grid: &[Vec<char>]) -> isize {
    used.insert(pos);
    let dirs = vec![
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
    ];
    let mut res = -1;
    for (add_i, add_j) in dirs {
        let new_i = pos.0 as isize + add_i;
        let new_j = pos.1 as isize + add_j;
        if new_i < 0 || new_i >= size.0 as isize || new_j < 0 || new_j >= size.1 as isize {
            continue;
        }
        let new_pos = (new_i as usize, new_j as usize);
        if grid[new_pos.0][new_pos.1] == '#' {
            continue;
        }
        if new_pos == start {
            if k + 1 >= 3 {
                chmax!(res, k + 1);
            }
            continue;
        }
        if used.contains(&new_pos) {
            continue;
        }
        chmax!(res, dfs(new_pos, k + 1, used, start, size, grid));
    }
    used.remove(&pos);
    res
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut res = -1;
    let mut used = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                continue;
            }
            chmax!(res, dfs((i, j), 0, &mut used, (i, j), (h, w), &grid));
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
