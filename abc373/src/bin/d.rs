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
        edges: [(Usize1, Usize1, i64); M]
    };

    let mut graph = vec![vec![]; N];
    for &(u, v, c) in &edges {
        graph[u].push((v, c));
        graph[v].push((u, -c));
    }

    let mut ans: Vec<Option<i64>> = vec![None; N];
    for u in 0..N {
        if ans[u].is_some() {
            continue;
        }
        ans[u] = Some(0);
        dfs(u, &graph, &mut ans);
    }
    println!("{}", ans.iter().map(|it| it.unwrap()).join(" "));
}

fn dfs(u: usize, graph: &Vec<Vec<(usize, i64)>>, ans: &mut Vec<Option<i64>>) {
    for &(v, c) in &graph[u] {
        if ans[v].is_some() {
            continue;
        }
        let new_cost = ans[u].unwrap() + c;
        ans[v] = Some(new_cost);
        dfs(v, graph, ans);
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
