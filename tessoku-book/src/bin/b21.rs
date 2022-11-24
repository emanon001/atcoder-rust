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
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn dfs(l: usize, r: usize, s: &[char], dp: &mut Vec<Vec<Option<usize>>>) -> usize {
    if l >= r {
        let res = 0;
        dp[l][r] = res.into();
        return res;
    }

    // 1文字
    if l + 1 == r {
        let res = 1;
        dp[l][r] = res.into();
        return res;
    }

    if let Some(res) = dp[l][r] {
        return res;
    }

    let mut res = 0;

    if s[l] == s[r - 1] {
        chmax!(res, dfs(l + 1, r - 1, s, dp) + 2);
    }
    chmax!(res, dfs(l + 1, r, s, dp));
    chmax!(res, dfs(l, r - 1, s, dp));

    dp[l][r] = res.into();
    res
}

fn solve() {
    input! {
        n: usize,
        s: Chars
    };

    let mut dp = vec![vec![None; n + 1]; n + 1];
    let res = dfs(0, n, &s, &mut dp);
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
