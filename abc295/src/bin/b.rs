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
        r: usize, c: usize,
        b: [Chars; r]
    };

    let mut ans = vec![vec![' '; c]; r];
    for i in 0..r {
        for j in 0..c {
            ans[i][j] = b[i][j];
            for add_i in -9_isize..=9 {
                for add_j in -9_isize..=9 {
                    let new_i = i as isize + add_i;
                    let new_j = j as isize + add_j;
                    if new_i < 0 || new_i >= r as isize || new_j < 0 || new_j >= c as isize {
                        continue;
                    }
                    let b_i = b[new_i as usize][new_j as usize];
                    if let Some(d) = b_i.to_digit(10) {
                        let dist = (i as isize - new_i).abs() + (j as isize - new_j).abs();
                        if dist > d as isize {
                            continue;
                        }
                        ans[i][j] = '.';
                    }
                }
            }
        }
    }
    for row in ans {
        println!("{}", row.iter().join(""));
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
