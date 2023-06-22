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
        n: usize, m: usize,
        xv: [usize; n],
        cyv: [(usize, usize); m]
    };

    let bonus_map = cyv.into_iter().collect::<HashMap<_, _>>();
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..n {
        for j in 0..=i {
            // 表
            chmax!(
                dp[i + 1][j + 1],
                dp[i][j] + xv[i] + bonus_map.get(&(j + 1)).unwrap_or(&0)
            );

            // 裏
            chmax!(dp[i + 1][0], dp[i][j]);
        }
    }
    let res = dp[n].iter().max().unwrap();
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
