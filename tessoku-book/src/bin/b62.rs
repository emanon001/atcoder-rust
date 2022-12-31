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
    let mut path = Vec::new();
    dfs(0, &mut path, &mut visited, &graph);
}

fn dfs(u: usize, path: &mut Vec<usize>, visited: &mut [bool], graph: &[Vec<usize>]) {
    visited[u] = true;
    path.push(u);
    if u == graph.len() - 1 {
        println!("{}", path.iter().map(|v| v + 1).join(" "));
        return;
    }
    for &v in &graph[u] {
        if visited[v] {
            continue;
        }
        dfs(v, path, visited, graph);
        path.pop();
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
