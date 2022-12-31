#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut visited = vec![false; n];
    dfs(0, &graph, &mut visited);
    let res = if visited.into_iter().all(|b| b) {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };
    println!("{}", res);
}

fn dfs(u: usize, graph: &[Vec<usize>], visited: &mut [bool]) {
    visited[u] = true;
    for &v in &graph[u] {
        if visited[v] {
            continue;
        }
        dfs(v, graph, visited);
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
