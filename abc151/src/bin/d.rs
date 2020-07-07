#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn move_count(grid: &Vec<Vec<char>>, h: usize, w: usize, start: (usize, usize)) -> usize {
    let di = vec![-1, 0, 1, 0];
    let dj = vec![0, 1, 0, -1];
    let mut visited = HashSet::new();
    let mut que = VecDeque::new();
    visited.insert(start);
    que.push_back((start, 0));
    let mut res = 0;
    while let Some(((i, j), cost)) = que.pop_front() {
        for d in 0..4 {
            let new_i = (i as isize) + di[d];
            let new_j = (j as isize) + dj[d];
            if new_i < 0 || new_i >= (h as isize) || new_j < 0 || new_j >= (w as isize) {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if grid[new_i][new_j] == '#' {
                continue;
            }
            let new_pos = (new_i, new_j);
            if visited.contains(&new_pos) {
                continue;
            }
            visited.insert(new_pos);
            let new_cost = cost + 1;
            que.push_back((new_pos, new_cost));
            res = res.max(new_cost);
        }
    }
    res
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != '#' {
                res = res.max(move_count(&grid, h, w, (i, j)));
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
