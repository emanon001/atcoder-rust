#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn score(path: &[(usize, usize)], grid: &[Vec<i32>]) -> i32 {
    let mut res = 0;
    for &(i, j) in path {
        res += grid[i][j];
    }
    res
}

fn solve() {
    input! {
        si: usize, sj: usize,
        tgrid: [[usize; 50]; 50],
        pgrid: [[i32; 50]; 50],
    };

    let mut res = Vec::new();
    let mut used = vec![false; 50 * 50];
    used[tgrid[si][sj]] = true;
    let dirs = vec![
        (-1, 0, 'U'),
        (0, 1, 'R'),
        (1, 0, 'D'),
        (0, -1, 'L')
    ];
    let mut cur_pos = (si, sj);
    loop {
        let mut point = -1;
        let mut dir = &'x';
        let mut next_pos = None;
        for (add_i, add_j, d) in &dirs {
            let new_i = cur_pos.0 as isize + add_i;
            if new_i < 0 || new_i >= 50 {
                continue;
            }
            let new_j = cur_pos.1 as isize + add_j;
            if new_j < 0 || new_j >= 50 {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            if used[tgrid[new_i][new_j]] {
                continue;
            }
            if pgrid[new_i][new_j] > point {
                point = pgrid[new_i][new_j];
                next_pos = Some((new_i, new_j));
                dir = d;
            }
        }
        if next_pos.is_none() {
            break;
        }
        let npos = next_pos.unwrap();
        used[tgrid[npos.0][npos.1]] = true;
        res.push(dir);
        cur_pos = npos;
    }

    println!("{}", res.iter().join(""));
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
