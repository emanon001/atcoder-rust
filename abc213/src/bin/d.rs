#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(u: usize, p: usize, res: &mut Vec<usize>, graph: &[BTreeSet<usize>]) {
    res.push(u + 1);
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        dfs(v, u, res, graph);
        res.push(u + 1);
    }
}

fn solve() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![BTreeSet::new(); n];
    for (u, v) in edges {
        graph[u].insert(v);
        graph[v].insert(u);
    }
    let mut res = Vec::new();
    dfs(0, n, &mut res, &graph);
    println!("{}", res.iter().join(" "));
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
