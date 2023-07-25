#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, abv: &[(i64, i64)], d: i64, visited: &mut Vec<bool>) {
    if visited[i] {
        return;
    }
    visited[i] = true;
    let (a1, a2) = abv[i];
    for (j, (b1, b2)) in abv.iter().copied().enumerate() {
        if (a1 - b1) * (a1 - b1) + (a2 - b2) * (a2 - b2) <= d * d {
            if visited[j] {
                continue;
            }
            dfs(j, abv, d, visited);
        }
    }
}

fn solve() {
    input! {
        n: usize, d: i64,
        xyv: [(i64, i64); n]
    };

    let mut visited = vec![false; n];
    dfs(0, &xyv, d, &mut visited);
    for b in visited {
        let res = if b { "Yes" } else { "No" };
        println!("{}", res);
    }
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
