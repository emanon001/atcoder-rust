#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(u: usize, p: usize, used: &mut[usize], res: &mut Vec<usize>, graph: &[Vec<usize>], cv: &[usize]) {
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        if used[cv[v]] == 0 {
            res.push(v + 1);
        }
        used[cv[v]] += 1;
        dfs(v, u, used, res, graph, cv);
        used[cv[v]] -= 1;
    }
}

fn solve() {
    input! {
        n: usize,
        cv: [Usize1; n],
        edges: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut used = vec![0; 10.pow(5)];
    used[cv[0]] += 1;
    let mut res = vec![1];
    dfs(0, n + 1, &mut used, &mut res, &graph, &cv);
    res.sort();
    for u in res {
        println!("{}", u);
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
