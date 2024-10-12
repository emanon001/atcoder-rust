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
        N: usize, S: usize,
        AB: [(usize, usize); N],
    };

    let mut dp = vec![vec![vec![false; 2]; S + 1]; N + 1];
    dp[0][0][0] = true;
    for i in 0..N {
        let (a, b) = AB[i];
        for j in 0..=S {
            for k in 0..2 {
                if !dp[i][j][k] {
                    continue;
                }
                if j + a <= S {
                    dp[i + 1][j + a][0] = true;
                }
                if j + b <= S {
                    dp[i + 1][j + b][1] = true;
                }
            }
        }
    }
    if !dp[N][S][0] && !dp[N][S][1] {
        println!("Impossible");
        return;
    }
    let mut ans = VecDeque::new();
    let mut s = S;
    for i in (1..=N).rev() {
        for k in 0..2 {
            if dp[i][s][k] {
                ans.push_front(if k == 0 { "A" } else { "B" });
                s -= if k == 0 { AB[i - 1].0 } else { AB[i - 1].1 };
                break;
            }
        }
    }
    println!("{}", ans.iter().join(""));
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
