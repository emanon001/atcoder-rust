#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        edges: [(Usize1, Usize1); N - 1]
    };

    let mut graph = vec![vec![]; N];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut visited = vec![false; N];
    let ans = if traverse(0, N, &graph, &mut visited) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

fn traverse(u: usize, p: usize, graph: &[Vec<usize>], visited: &mut Vec<bool>) -> bool {
    visited[u] = true;
    let mut res = true;
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        if v != p && visited[v] {
            return false;
        }
        res &= traverse(v, u, graph, visited);
    }
    res
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
