#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(u: usize, p: usize, graph: &[Vec<usize>], xv: &[i64], dp: &mut Vec<Vec<i64>>) -> Vec<i64> {
    let mut res = vec![xv[u]];
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        for a in dfs(v, u, graph, xv, dp) {
            res.push(a);
        }
    }
    res.sort_by(|a, b| b.cmp(a));
    res = res.into_iter().take(20).collect();
    dp[u] = res.clone();
    res
}

fn solve() {
    input! {
        n: usize, q: usize,
        xv: [i64; n],
        edges: [(Usize1, Usize1); n - 1],
        queries: [(Usize1, Usize1); q]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut res = vec![Vec::new(); n];
    dfs(0, n + 1, &graph, &xv, &mut res);
    for (v, k) in queries {
        println!("{}", res[v][k]);
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
