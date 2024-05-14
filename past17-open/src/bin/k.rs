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
        S: Chars,
    };

    let mut dp = vec![vec![None; 10]; N + 1];
    dp[0][0] = Some((0, 0)); // dummy
    for (i, ch) in S.into_iter().enumerate() {
        for j in 0..10 {
            // j は mod 10

            // 使用できる数字の範囲
            let (from, to) = if let Some(a) = ch.to_digit(10) {
                let a = a as usize;
                (a, a)
            } else {
                (0, 9)
            };

            // 今の桁の数字を決める
            for k in from..=to {
                let m = (j + (i + 1) * k) % 10;
                match (dp[i][j], dp[i + 1][m]) {
                    (Some(_), None) => {
                        dp[i + 1][m] = Some((j, k));
                    }
                    _ => {
                        // ignore
                    }
                }
            }
        }
    }
    // eprintln!("{:?}", dp);
    if dp[N][0].is_none() {
        println!("No");
        return;
    }
    println!("Yes");
    let mut ans = VecDeque::new();
    let mut i = 0;
    for j in (1..=N).rev() {
        let (prev, k) = dp[j][i].unwrap();
        ans.push_front(k);
        i = prev;
    }
    println!("{}", ans.into_iter().join(""));
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
