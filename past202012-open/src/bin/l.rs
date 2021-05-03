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

fn dfs(l: usize, r: usize, dp: &mut [Vec<i64>], s: &[char], t: &[char]) -> i64 {
    // [l, r)
    if dp[l][r] != -1 {
        return dp[l][r];
    }
    if l + 3 > r  {
        return 0;
    }
    let mut res = 0;
    chmax!(res, dfs(l + 1, r, dp, s, t));
    chmax!(res, dfs(l, r - 1, dp, s, t));

    for m in l + 1..r {
        let x = dfs(l, m, dp, s, t) + dfs(m, r, dp, s, t);
        chmax!(res, x);
        if s[l] == t[0] && s[m] == t[1] && s[r - 1] == t[2] {
            let a = dfs(l + 1, m, dp, s, t);
            let b = dfs(m + 1, r - 1, dp, s, t);
            if a == (m - l - 1) as i64 && b == ((r - m) as i64 - 2).max(0) {
                let x = (r - l) as i64;
                chmax!(res, x);
            }
        }
    }
    dp[l][r] = res;
    // println!("l: {}, r: {}, s: {}, res: {}", l, r, s[l..r].iter().join(""), dp[l][r]);
    dp[l][r]
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    };

    let mut dp = vec![vec![-1; n + 1]; n + 1];
    let res = dfs(0, n, &mut dp, &s, &t) / 3;
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
