#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        h: usize, w: usize,
        a: [Chars; h],
        b: [Chars; h],
    };

    for s in 0..h {
        for t in 0..w {
            if b == shift(s, t, &a, h, w) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}

fn shift(s: usize, t: usize, grid: &[Vec<char>], h: usize, w: usize) -> Vec<Vec<char>> {
    let mut ans = vec![vec![' '; w]; h];
    for i in 0..h {
        for j in 0..w {
            ans[(i + s) % h][(j + t) % w] = grid[i][j];
        }
    }
    ans
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
