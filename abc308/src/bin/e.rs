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
        n: usize,
        av: [usize; n],
        s: Chars
    };

    // dp[i][j][k][l] = count
    // i: i番目まで見た
    // j: Mに対応する値(0, 1, 2, 3 = 未設定)
    // k: Eに対応する値(0, 1, 2, 3)
    // l: Xに対応する値(0, 1, 2, 3)
    let mut dp = vec![vec![vec![vec![0_u64; 4]; 4]; 4]; n + 1];
    dp[0][3][3][3] = 1;
    for i in 0..n {
        let a = av[i];
        let ch = s[i];
        if ch == 'M' {
            dp[i + 1][a][3][3] += 1;
        }
        if ch == 'E' {
            for j in 0..3 {
                dp[i + 1][j][a][3] += dp[i][j][3][3];
            }
        }
        if ch == 'X' {
            for j in 0..3 {
                for k in 0..3 {
                    dp[i + 1][j][k][a] += dp[i][j][k][3];
                }
            }
        }
        for j in 0..4 {
            for k in 0..4 {
                for l in 0..4 {
                    dp[i + 1][j][k][l] += dp[i][j][k][l];
                }
            }
        }
    }
    // eprintln!("{:?}", dp);
    let mut ans = 0_u64;
    for j in 0..3 {
        for k in 0..3 {
            for l in 0..3 {
                for add in 0..=4 {
                    if add != j && add != k && add != l {
                        ans += add as u64 * dp[n][j][k][l];
                        break;
                    }
                }
            }
        }
    }
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
