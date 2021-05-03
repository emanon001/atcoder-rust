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
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

const INF: i64 = 1_i64 << 60;

fn dfs(l: usize, r: usize, dp: &mut [Vec<i64>], av: &[i64]) -> i64 {
    // [l, r)
    if dp[l][r] != -1 {
        return dp[l][r];
    }
    if l == r {
        return 0;
    }
    if l + 1 == r {
        return INF;
    }
    let mut res = INF;
    let x = dfs(l + 1, r - 1, dp, av) + (av[l] - av[r - 1]).abs();
    chmin!(res, x);
    for m in l + 1..r {
        let x = dfs(l, m, dp, av) + dfs(m, r, dp, av);
        chmin!(res, x);
    }
    dp[l][r] = res;
    dp[l][r]
}

fn solve() {
    input! {
        n: usize,
        av: [i64; 2 * n]
    };

    let mut dp = vec![vec![-1; 2 * n + 1]; 2 * n + 1];
    let res = dfs(0, 2 * n, &mut dp, &av);
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
