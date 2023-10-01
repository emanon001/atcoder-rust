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
    visited[0] = true;
    let ans = dfs(0, &mut visited, &graph);
    println!("{}", ans);
}

const MAX_ANS: usize = 1000000;

fn dfs(u: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) -> usize {
    let mut res = 1;
    for &v in &graph[u] {
        if visited[v] {
            continue;
        }
        visited[v] = true;
        res = (res + dfs(v, visited, graph)).min(MAX_ANS);
        visited[v] = false;
        if res == MAX_ANS {
            break;
        }
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
