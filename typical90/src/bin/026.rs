#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(u: usize, color: isize, colors: &mut Vec<isize>, graph: &[Vec<usize>]) {
    colors[u] = color;
    for &v in &graph[u] {
        if colors[v] != -1 {
            continue;
        }
        dfs(v, (color + 1) % 2, colors, graph);
    }
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
    let mut colors = vec![-1; n];
    dfs(0, 0, &mut colors, &graph);
    let c = if colors.iter().filter(|&c| *c == 0).count() >= n / 2 {
        0
    } else {
        1
    };
    let mut res = Vec::new();
    for u in 0..n {
        if colors[u] == c {
            res.push(u + 1);
        }
    }
    let res = res.into_iter().take(n / 2).join("\n");
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
