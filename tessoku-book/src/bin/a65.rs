#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        av: [Usize1; n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for u in 1..n {
        let v = av[u - 1];
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut dp = vec![0; n];
    dfs(0, 1 << 30, &graph, &mut dp);
    println!("{}", dp.iter().join(" "));
}

fn dfs(u: usize, parent: usize, graph: &[Vec<usize>], dp: &mut [usize]) -> usize {
    let mut res = 0;
    for &v in &graph[u] {
        if v == parent {
            continue;
        }
        res += dfs(v, u, graph, dp) + 1;
    }
    dp[u] = res;
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
