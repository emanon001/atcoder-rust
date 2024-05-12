#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        ABC: [(i64, usize, usize); N],
    };

    let mut lectures = HashMap::new();
    for (a, b, c) in &ABC {
        lectures.entry(b).or_insert(vec![]).push((*a, *c));
    }

    let INF = 1_i64 << 60;
    let mut dp = vec![vec![INF; M + 1]; lectures.len() + 1];
    dp[0][0] = 0_i64;
    for (i, (_, lectures)) in lectures.iter().enumerate() {
        for j in 0..=M {
            chmin!(dp[i + 1][j], dp[i][j]);
            for &(a, c) in lectures {
                let new_j = (j + c).min(M);
                chmin!(dp[i + 1][new_j], dp[i][j] + a);
            }
        }
    }
    let ans = if dp[lectures.len()][M] == INF {
        -1
    } else {
        dp[lectures.len()][M]
    };
    println!("{}", ans);
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
