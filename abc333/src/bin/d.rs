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

    let mut depth_list = Vec::new();
    for &v in &graph[0] {
        depth_list.push(dfs(v, 0, &graph));
    }
    let ans = depth_list
        .into_iter()
        .sorted()
        .take(graph[0].len() - 1)
        .sum::<usize>()
        + 1;
    println!("{}", ans);
}

fn dfs(u: usize, parent: usize, graph: &Vec<Vec<usize>>) -> usize {
    let mut ans = 1;
    for &v in &graph[u] {
        if v == parent {
            continue;
        }
        ans += dfs(v, u, graph);
    }
    ans
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
