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

fn dfs(
    pos: (usize, usize),
    path: &mut Vec<(usize, usize, char)>,
    used: &mut [bool],
    score: &mut i32,
    check_count: &mut u32,
    res: &mut (i32, Vec<(usize, usize, char)>),
    tgrid: &[Vec<usize>],
    pgrid: &[Vec<i32>],
    now: Instant,
) {
    if check_count == &1000 {
        let duration = Instant::now() - now;
        let stop = duration >= Duration::from_millis(1980);
        if stop {
            if *score > res.0 {
                res.0 = *score;
                res.1 = path.clone();
            }
            return;
        }
        *check_count = 0;
    }
    *check_count += 1;
    let dirs = vec![
        (-1, 0, 'U'),
        (0, 1, 'R'),
        (1, 0, 'D'),
        (0, -1, 'L')
    ];
    // ポイントの高い順に試す
    let mut cand = Vec::new();
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
        cand.push((new_i, new_j, d));
    }
    let moved = cand.len() > 0;
    cand.sort_by_key(|(i, j, _)| -pgrid[*i][*j]);
    for (new_i, new_j, d) in cand {
        let new_pos = (new_i, new_j);
        *score += pgrid[new_pos.0][new_pos.1];
        path.push((new_i, new_j, *d));
        used[tgrid[new_i][new_j]] = true;
        dfs(new_pos, path, used, score, check_count, res, tgrid, pgrid, now);
        *score -= pgrid[new_pos.0][new_pos.1];
        path.pop();
        used[tgrid[new_i][new_j]] = false;
    }
    if !moved {
        if *score > res.0 {
            res.0 = *score;
            res.1 = path.clone();
        }
    }
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
    let mut check_count = 0;
    let mut score = pgrid[si][sj];
    let mut res1 = (0, vec![(si, sj, ' ')]);
    dfs((si, sj), &mut path, &mut used, &mut score, &mut check_count, &mut res1, &tgrid, &pgrid, now);
    let res = res1.1.iter().map(|(_, _, d)| *d).join("");
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
