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
        N: usize, M: usize,
        S: [Chars; N],
    };

    for i in 0..N {
        let mut ans = vec![];
        for j in 0..M {
            ans.push(count(i, j, &S));
        }
        println!("{}", ans.iter().join(""));
    }
}

fn count(i: usize, j: usize, s: &Vec<Vec<char>>) -> usize {
    let mut res = 0;
    if s[i][j] == '#' {
        res += 1;
    }
    for &(di, dj) in &ALL_DIRS {
        let i = i as isize + di;
        let j = j as isize + dj;
        if i < 0 || i >= s.len() as isize || j < 0 || j >= s[0].len() as isize {
            continue;
        }
        if s[i as usize][j as usize] == '#' {
            res += 1;
        }
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
