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
    let mut ans = 0_i64;
    dfs(0, N, &graph, &mut ans);
    println!("{}", ans);
}

fn dfs(u: usize, p: usize, g: &Vec<Vec<usize>>, ans: &mut i64) -> i64 {
    let n = g.len() as i64;
    let mut children_count = 0_i64;
    for &v in &g[u] {
        if v == p {
            continue;
        }
        children_count += dfs(v, u, g, ans);
    }
    let parent_direction_count = n - 1 - children_count;
    *ans += parent_direction_count;
    *ans += children_count * parent_direction_count;
    children_count + 1
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
