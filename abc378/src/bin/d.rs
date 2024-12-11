#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize, K: usize,
        S: [Chars; H],
    };

    let mut ans = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                continue;
            }
            let mut visited = HashSet::new();
            visited.insert((i, j));
            ans += dfs(i, j, &S, H, W, K, &mut visited);
        }
    }
    println!("{}", ans);
}

fn dfs(
    i: usize,
    j: usize,
    s: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    k: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> usize {
    if visited.len() == k + 1 {
        return 1;
    }

    let mut res = 0;
    for (di, dj) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
            continue;
        }
        let ni = ni as usize;
        let nj = nj as usize;
        if s[ni][nj] == '#' {
            continue;
        }
        if visited.contains(&(ni, nj)) {
            continue;
        }
        visited.insert((ni, nj));
        res += dfs(ni, nj, s, h, w, k, visited);
        visited.remove(&(ni, nj));
    }
    res
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
