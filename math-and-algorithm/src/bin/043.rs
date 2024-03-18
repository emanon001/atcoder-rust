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
        N: usize, M: usize,
        AB: [(Usize1, Usize1); M],
    };

    let mut graph = vec![vec![]; N];
    for (u, v) in AB {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut visited = vec![false; N];
    traverse(0, &mut visited, &graph);
    let connected = visited.into_iter().all(|x| x);
    let ans = if connected {
        "The graph is connected."
    } else {
        "The graph is not connected."
    };
    println!("{}", ans);
}

fn traverse(u: usize, visited: &mut Vec<bool>, graph: &Vec<Vec<usize>>) {
    if visited[u] {
        return;
    }
    visited[u] = true;
    for &v in &graph[u] {
        traverse(v, visited, graph);
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
