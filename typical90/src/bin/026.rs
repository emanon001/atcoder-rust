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
        edges: [(Usize1, Usize1); N - 1],
    };

    let mut graph = vec![vec![]; N];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut ans = vec![];
    dfs(0, N, &graph, &mut ans);
    let ans = ans.iter().take(N / 2).join(" ");
    println!("{}", ans);
}

fn dfs(u: usize, p: usize, g: &Vec<Vec<usize>>, ans: &mut Vec<usize>) -> bool {
    let mut is_children_choose = false;
    for v in &g[u] {
        if *v == p {
            continue;
        }
        is_children_choose |= dfs(*v, u, g, ans);
    }
    let res = !is_children_choose;
    if res {
        ans.push(u + 1);
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
