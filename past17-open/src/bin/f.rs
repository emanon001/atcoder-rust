#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const MOD: i64 = 998244353;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        P: [Usize1; N - 1],
        S: [String; N],
    };

    let mut graph = vec![vec![]; N];
    for (i, &p) in P.iter().enumerate() {
        graph[p].push(i + 1);
    }
    let ans = dfs(0, &graph, &S);
    println!("{}", ans);
}

fn dfs(u: usize, graph: &[Vec<usize>], value: &[String]) -> i64 {
    (match value[u].as_ref() {
        "x" => dfs(graph[u][0], graph, value) * dfs(graph[u][1], graph, value),
        "+" => dfs(graph[u][0], graph, value) + dfs(graph[u][1], graph, value),
        x => x.parse::<i64>().unwrap(),
    }) % MOD
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
