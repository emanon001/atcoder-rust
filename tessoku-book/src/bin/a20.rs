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

fn dfs(i: usize, j: usize, s: &[char], t: &[char], dp: &mut Vec<Vec<Option<usize>>>) -> usize {
    if i == 0 || j == 0 {
        let res = 0;
        dp[i][j] = res.into();
        return res;
    }

    if let Some(res) = dp[i][j] {
        return res;
    }

    let mut res = 0;
    if s[i - 1] == t[j - 1] {
        chmax!(res, dfs(i - 1, j - 1, s, t, dp) + 1);
    }
    chmax!(res, dfs(i - 1, j, s, t, dp));
    chmax!(res, dfs(i, j - 1, s, t, dp));
    dp[i][j] = res.into();
    res
}

fn solve() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut dp = vec![vec![None; t.len() + 1]; s.len() + 1];
    let res = dfs(s.len(), t.len(), &s, &t, &mut dp);
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
