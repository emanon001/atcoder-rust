#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        h: usize, w: usize, k: usize,
        grid: [Chars; h]
    };

    let mut res = 0;
    for bits in 0..1 << h {
        let row_choice_count = format!("{:b}", bits)
            .chars()
            .filter(|ch| ch == &'1')
            .count();
        if row_choice_count > k {
            continue;
        }
        let mut row_score = 0;
        for (i, row) in grid.iter().enumerate() {
            for ch in row {
                if ch == &'#' || (bits >> i) & 1 == 1 {
                    row_score += 1;
                }
            }
        }
        let mut col_scores = Vec::new();
        for j in 0..w {
            let mut score = 0;
            for i in 0..h {
                if grid[i][j] != '#' && (bits >> i) & 1 == 0 {
                    score += 1;
                }
            }
            col_scores.push(score);
        }
        col_scores.sort_by(|a, b| b.cmp(a));
        let score = row_score
            + col_scores
                .into_iter()
                .take(k - row_choice_count)
                .sum::<i32>();
        chmax!(res, score);
    }
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
