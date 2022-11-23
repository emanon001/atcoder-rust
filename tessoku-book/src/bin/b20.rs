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

fn dfs(si: usize, ti: usize, s: &[char], t: &[char], dp: &mut Vec<Vec<Option<usize>>>) -> usize {
    if let Some(res) = dp[si][ti] {
        return res;
    }

    // dp[si][ti] = si と ti が対応する場合の最小コスト
    if si == 0 || ti == 0 {
        let res = (si as isize - ti as isize).abs() as usize;
        dp[si][ti] = res.into();
        return res;
    }

    let mut res = 1 << 30;
    if s[si - 1] == t[ti - 1] {
        chmin!(res, dfs(si - 1, ti - 1, s, t, dp));
    }
    // S[i]を削除
    chmin!(res, dfs(si - 1, ti, s, t, dp) + 1);
    // S[i]の文字を変更
    chmin!(res, dfs(si - 1, ti - 1, s, t, dp) + 1);
    // S[i]の後ろに文字を追加
    chmin!(res, dfs(si, ti - 1, s, t, dp) + 1);
    dp[si][ti] = res.into();
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
