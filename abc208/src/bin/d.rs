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

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, i64); m]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v, c) in edges {
        graph[u].push((v, c));
    }

    let inf = 1_i64 << 60;
    let mut dp = vec![vec![inf; n]; n];
    for i in 0..n {
        dp[i][i] = 0;
    }

    for u in 0..n {
        for &(v, w) in &graph[u] {
            chmin!(dp[u][v], w);
        }
    }

    let mut res = 0_i64;
    for k in 0..n {
        // update
        for i in 0..dp.len() {
            for j in 0..dp.len() {
                chmin!(dp[i][j], dp[i][k] + dp[k][j]);
            }
        }
        // sum
        for s in 0..n {
            for t in 0..n {
                let cost = dp[s][t];
                res += if cost == inf { 0 } else { cost };
            }
        }
    }
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
