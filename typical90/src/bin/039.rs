#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(u: usize, p: usize, graph: &[Vec<usize>], n: usize) -> (u64, u64) {
    let mut cost_sum = 0_u64;
    let mut vn_sum = 0_u64;
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        let (cost, vn) = dfs(v, u, graph, n);
        let vn = vn + 1; // v
        cost_sum += cost + vn * (n as u64 - vn);
        vn_sum += vn;
    }
    (cost_sum, vn_sum)
}

fn solve() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let (res, _) = dfs(0, n, &graph, n);
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
