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
        N: usize,
    };
    let carpet = carpet(N);
    let ans = carpet.into_iter().map(|row| row.iter().join("")).join("\n");
    println!("{}", ans);
}

fn carpet(k: usize) -> Vec<Vec<char>> {
    if k == 0 {
        return vec![vec!['#']];
    }
    let size = 3.pow(k as u32);
    let mut res = vec![vec!['.'; size]; size];
    let child_size = 3.pow(k as u32 - 1);
    let child_carpet = carpet(k - 1);
    for i in 0..3 {
        for j in 0..3 {
            for i2 in 0..child_size {
                for j2 in 0..child_size {
                    res[i * child_size + i2][j * child_size + j2] = child_carpet[i2][j2];
                }
            }
        }
    }
    for i in 0..child_size {
        for j in 0..child_size {
            res[child_size + i][child_size + j] = '.'
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
