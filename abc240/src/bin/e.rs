#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(graph: &[Vec<usize>], u: usize, p: usize, max: usize, res: &mut [(usize, usize)]) -> usize {
    let l = max;
    let mut r = max;
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        r = dfs(graph, v, u, r, res);
    }
    r = (r - 1).max(l);
    res[u] = (l, r);
    r + 1
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
    let mut res = vec![(0, 0); n];
    dfs(&graph, 0, 0, 1, &mut res);
    for (l, r) in res {
        println!("{} {}", l, r);
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
