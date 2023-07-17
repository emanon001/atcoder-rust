#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(
    i: i64,
    j: i64,
    id: usize,
    pos_set: &HashSet<(i64, i64)>,
    visited: &mut HashMap<(i64, i64), usize>,
) {
    if visited.contains_key(&(i, j)) {
        return;
    }
    visited.insert((i, j), id);
    let dirs = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];
    for (di, dj) in dirs {
        let new_i = i + di;
        let new_j = j + dj;
        if !pos_set.contains(&(new_i, new_j)) {
            continue;
        }
        if visited.contains_key(&(new_i, new_j)) {
            continue;
        }
        dfs(new_i, new_j, id, pos_set, visited);
    }
}

fn solve() {
    input! {
        n: usize,
        xyv: [(i64, i64); n]
    };

    let pos_set = xyv.iter().copied().collect::<HashSet<_>>();
    let mut visited = HashMap::new();
    for (id, (x, y)) in xyv.into_iter().enumerate() {
        dfs(x, y, id, &pos_set, &mut visited);
    }
    let res = visited.values().unique().count();
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
