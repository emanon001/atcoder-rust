#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn rotate90<T: Clone>(grid: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!grid.is_empty());
    (0..grid[0].len())
        .map(|col_i| {
            (0..grid.len())
                .rev()
                .map(|row_i| grid[row_i][col_i].clone())
                .collect::<Vec<_>>()
        })
        .collect()
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        S: [Chars; N]
    };

    let max_len = S.iter().map(|s| s.len()).max().unwrap();
    let mut s = S.clone();
    for i in 0..N {
        while s[i].len() < max_len {
            s[i].push('*');
        }
    }
    let s = rotate90(s);
    for l in s {
        println!(
            "{}",
            l.into_iter().collect::<String>().trim_end_matches('*')
        );
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
