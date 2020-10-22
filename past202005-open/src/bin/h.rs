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
        n: usize, l: usize,
        xv: [usize; n],
        t1: i64, t2: i64, t3: i64
    };

    let x_set = xv.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![1_i64 << 60; l + 10];
    dp[0] = 0;
    for i in 0..l {
        let t3_cost = if x_set.contains(&i) { t3 } else { 0 };

        // 行動 1: 距離 1 を走って進む。
        dp[i + 1] = dp[i + 1].min(dp[i] + t1 + t3_cost);

        // 行動 2: 距離 0.5 を走って進み、ジャンプして距離 1 を進み、また距離 0.5 を走って進む。
        // 合計で 2 の距離を進むことになる。
        if i + 1 == l {
            dp[i + 1] = dp[i + 1].min(dp[i] + (t1 + t2) / 2 + t3_cost);
        }
        dp[i + 2] = dp[i + 2].min(dp[i] + t1 + t2 + t3_cost);

        // 行動 3: 距離 0.5 を走って進み、ジャンプして距離 3 を進み、また距離 0.5 を走って進む。
        // 合計で 4 の距離を進むことになる。
        if i + 1 == l {
            dp[i + 1] = dp[i + 1].min(dp[i] + (t1 + t2) / 2 + t3_cost);
        }
        if i + 2 == l {
            dp[i + 2] = dp[i + 2].min(dp[i] + t1 / 2 + t2 + t2 / 2 + t3_cost);
        }
        if i + 3 == l {
            dp[i + 3] = dp[i + 3].min(dp[i] + t1 / 2 + t2 * 2 + t2 / 2 + t3_cost);
        }
        dp[i + 4] = dp[i + 4].min(dp[i] + t1 + t2 * 3 + t3_cost);
    }
    println!("{}", dp[l]);
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
