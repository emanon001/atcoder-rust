#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, j: usize, k: usize, av: &[usize], dp: &mut [Vec<Vec<Option<usize>>>]) -> usize {
    if let Some(res) = dp[i][j][k] {
        return res;
    }

    let n = av.len();
    let new_k = (k + 1) % 2;
    if i == n - 1 {
        let res = av[j];
        dp[i][j][new_k] = res.into();
        return res;
    }

    let res = if k == 0 {
        dfs(i + 1, j, new_k, av, dp).max(dfs(i + 1, j + 1, new_k, av, dp))
    } else {
        dfs(i + 1, j, new_k, av, dp).min(dfs(i + 1, j + 1, new_k, av, dp))
    };
    dp[i][j][k] = res.into();
    res
}

fn solve() {
    input! {
        n: usize,
        av: [usize; n]
    };

    let mut dp: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; 2]; n]; n];
    let res = dfs(0, 0, 0, &av, &mut dp);
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
