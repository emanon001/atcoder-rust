#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(a: usize, parents: &Vec<Option<usize>>, dp: &mut Vec<Option<usize>>) -> usize {
    if let Some(res) = dp[a] {
        return res;
    }
    if a == 1 {
        let res = 0;
        dp[1] = res.into();
        return res;
    }
    if let Some(p) = parents[a] {
        let res = dfs(p, parents, dp) + 1;
        dp[a] = res.into();
        return res;
    } else {
        unreachable!();
    }
}

fn solve() {
    input! {
        n: usize,
        av: [usize; n]
    };

    let mut parents = vec![None; n * 2 + 10];
    for (i, a) in av.into_iter().enumerate() {
        let i = i + 1;
        parents[i * 2] = a.into();
        parents[i * 2 + 1] = a.into();
    }
    let mut dp = vec![None; n * 2 + 10];
    for k in 1..=n * 2 + 1 {
        let res = dfs(k, &parents, &mut dp);
        println!("{}", res);
    }
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
