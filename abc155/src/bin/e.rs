#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: Chars
    };

    let n = n
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as isize)
        .rev()
        .chain(vec![0])
        .collect::<Vec<_>>();
    let len = n.len();
    // dp[i][j] = 最小のコイン数
    // i: 後ろからi桁目まで見た
    // j: 0: 繰り下がりがない, 1: 繰り下がりがある
    let mut dp = vec![vec![1_isize << 30; 2]; len + 1];
    dp[0][0] = 0;
    for i in 0..len {
        let a = n[i];
        for j in 0..2 {
            for b in 0_isize..=9 {
                let to_i = i + 1;
                let x = b - if j == 0 { 0 } else { 1 };
                if x >= a {
                    let to_j = 0;
                    dp[to_i][to_j] = dp[to_i][to_j].min(dp[i][j] + b + (x - a));
                } else {
                    let to_j = 1;
                    dp[to_i][to_j] = dp[to_i][to_j].min(dp[i][j] + b + (x - a + 10));
                }
            }
        }
    }
    println!("{}", dp[len][0]);
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
