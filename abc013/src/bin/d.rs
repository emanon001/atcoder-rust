#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, j: usize, av: &[usize], dp: &mut [Vec<Option<usize>>]) -> usize {
    if i == av.len() {
        return j;
    }
    if av[i] == j || av[i] + 1 == j {
        let dir = if av[i] == j { 0 } else { 1 };
        return if let Some(res) = dp[i][dir] {
            res
        } else {
            let new_j = if av[i] == j { j + 1 } else { j - 1 };
            let res = dfs(i + 1, new_j, av, dp);
            dp[i][dir] = Some(res);
            res
        };
    } else {
        return dfs(i + 1, j, av, dp);
    }
}

fn solve() {
    input! {
        n: usize, m: usize, d: usize,
        av: [Usize1; m]
    };

    let mut dp = vec![vec![None; 2]; m];
    let mut to = vec![vec![0; n]; 30];
    for i in 0..n {
        let j = dfs(0, i, &av, &mut dp);
        to[0][i] = j;
    }
    for i in 0..29 {
        for j in 0..n {
            let k = to[i][to[i][j]];
            to[i + 1][j] = k;
        }
    }
    for i in 0..n {
        let mut rest = d;
        let mut cur = i;
        while rest > 0 {
            let mut c = 0;
            while 2.pow(c + 1) <= rest {
                c += 1;
            }
            let c = c as usize;
            cur = to[c][cur];
            rest -= 2.pow(c as u32);
        }
        println!("{}", cur + 1);
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
