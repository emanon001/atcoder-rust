#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use whiteread::parse_line;

fn read_line(width: usize) -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s[..width].to_string()
}

fn solve() {
    let (n, l): (usize, usize) = parse_line().unwrap();
    let width = 2 * n - 1;
    let mut grid = Vec::new();
    for _ in 0..l {
        let line = read_line(width).chars().into_iter().collect::<Vec<char>>();
        grid.push(line);
    }
    let mut goal = read_line(width).chars().into_iter().collect::<Vec<char>>();

    let grid = grid
        .into_iter()
        .map(|mut rows| {
            rows.reverse();
            rows
        })
        .rev()
        .collect::<Vec<_>>();
    goal.reverse();
    let mut pos = goal.into_iter().position(|ch| ch == 'o').unwrap();
    for line in grid {
        if pos > 0 && line[pos - 1] == '-' {
            pos -= 2;
        } else if pos < width - 1 && line[pos + 1] == '-' {
            pos += 2;
        }
    }
    let res = n - (pos / 2);
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
