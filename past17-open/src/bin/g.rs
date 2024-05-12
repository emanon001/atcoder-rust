#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

/// 上下左右 + 斜め (i, j)
pub const ALL_DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        G: [Chars; H],
        _: usize,
        S: Chars,
    };

    for i in 0..H {
        for j in 0..W {
            if G[i][j] == S[0] {
                let mut path = vec![(i, j)];
                if dfs((i, j), &G, &S, &mut path) {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}

fn dfs(pos: (usize, usize), g: &[Vec<char>], s: &[char], path: &mut Vec<(usize, usize)>) -> bool {
    if path.len() == s.len() {
        return true;
    }

    let (i, j) = pos;
    for (di, dj) in ALL_DIRS.iter() {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if ni < 0 || ni >= g.len() as isize || nj < 0 || nj >= g[0].len() as isize {
            continue;
        }
        let ni = ni as usize;
        let nj = nj as usize;
        let new_pos = (ni, nj);
        if path.contains(&new_pos) {
            continue;
        }
        if g[ni][nj] == s[path.len()] {
            path.push(new_pos);
            if dfs(new_pos, g, s, path) {
                return true;
            }
            path.pop();
        }
    }
    false
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
