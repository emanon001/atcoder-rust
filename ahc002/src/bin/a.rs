#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::time::{Instant, Duration};

fn score(path: &[(usize, usize, char)], grid: &[Vec<i32>]) -> i32 {
    let mut res = 0;
    for &(i, j, _) in path {
        res += grid[i][j];
    }
    res
}

fn dfs(
    pos: (usize, usize),
    path: &mut Vec<(usize, usize, char)>,
    depth: usize,
    used: &mut [bool],
    res: &mut (i32, Vec<(usize, usize, char)>),
    tgrid: &[Vec<usize>],
    pgrid: &[Vec<i32>],
    now: Instant
) {
    let duration = Instant::now() - now;
    let stop = duration >= Duration::from_millis(1970);
    if stop {
        let s = score(path, pgrid);
        if s > res.0 {
            res.0 = s;
            res.1 = path.clone();
        }
        return;
    }
    let dirs = vec![
        (-1, 0, 'U'),
        (0, 1, 'R'),
        (1, 0, 'D'),
        (0, -1, 'L')
    ];
    let mut moved = false;
    for (add_i, add_j, d) in &dirs {
        let new_i = pos.0 as isize + add_i;
        if new_i < 0 || new_i >= 50 {
            continue;
        }
        let new_j = pos.1 as isize + add_j;
        if new_j < 0 || new_j >= 50 {
            continue;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if used[tgrid[new_i][new_j]] {
            continue;
        }
        moved = true;
        let new_pos = (new_i, new_j);
        path.push((new_i, new_j, *d));
        used[tgrid[new_i][new_j]] = true;
        dfs(new_pos, path, depth + 1, used, res, tgrid, pgrid, now);
        path.pop();
        used[tgrid[new_i][new_j]] = false;
    }
    if !moved {
        let s = score(path, pgrid);
        if s > res.0 {
            res.0 = s;
            res.1 = path.clone();
        }
    }
}

fn move_to_max_point_adjacent_cells(si: usize, sj: usize, used: &mut [bool], tgrid: &[Vec<usize>], pgrid: &[Vec<i32>]) -> (i32, Vec<char>) {
    let mut score = 0_i32;
    let mut path = Vec::new();
    score += pgrid[si][sj];
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
        score += pgrid[npos.0][npos.1];
        used[tgrid[npos.0][npos.1]] = true;
        path.push(*dir);
        cur_pos = npos;
    }
    (score, path)
}

fn solve() {
    input! {
        si: usize, sj: usize,
        tgrid: [[usize; 50]; 50],
        pgrid: [[i32; 50]; 50],
    };

    let now = Instant::now();
    let mut path = Vec::new();
    let mut used = vec![false; 50 * 50];
    used[tgrid[si][sj]] = true;
    let mut res1 = (0, vec![(si, sj, ' ')]);
    dfs((si, sj), &mut path, 0,  &mut used, &mut res1, &tgrid, &pgrid, now);
    for &(i, j, _) in &res1.1 {
        used[tgrid[i][j]] = true;
    }
    let last = res1.1.last().unwrap();
    let res2 = move_to_max_point_adjacent_cells(last.0, last.1, &mut used, &tgrid, &pgrid);
    let res1_str = res1.1.iter().map(|(_, _, d)| *d).join("");
    let res = format!("{}{}", res1_str, res2.1.iter().join(""));
    println!("{}", res.trim());
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
