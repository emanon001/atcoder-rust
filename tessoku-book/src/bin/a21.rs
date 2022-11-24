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

fn dfs(l: usize, r: usize, pav: &[(usize, i64)], dp: &mut Vec<Vec<Option<i64>>>) -> i64 {
    if l >= r {
        let res = 0;
        dp[l][r] = res.into();
        return res;
    }

    if let Some(res) = dp[l][r] {
        return res;
    }

    let mut res = -(1 << 30);

    // 左端を取り除いた時
    let (p, a) = pav[l];
    let plus_score = if (l..r).contains(&p) { a } else { 0 };
    chmax!(res, dfs(l + 1, r, pav, dp) + plus_score);

    // 右端を取り除いた時
    let (p, a) = pav[r - 1]; // [l, r)
    let plus_score = if (l..r).contains(&p) { a } else { 0 };
    chmax!(res, dfs(l, r - 1, pav, dp) + plus_score);

    dp[l][r] = res.into();
    res
}

fn solve() {
    input! {
        n: usize,
        pav: [(Usize1, i64); n]
    };

    let mut dp = vec![vec![None; n + 1]; n + 1];
    let res = dfs(0, n, &pav, &mut dp);
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
