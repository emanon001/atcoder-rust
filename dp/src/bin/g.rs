#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![Vec::new(); n];
    let mut indegree = vec![0; n];
    for (u, v) in edges {
        graph[u].push(v);
        indegree[v] += 1;
    }
    let mut que = VecDeque::new();
    for i in 0..n {
        if indegree[i] == 0 {
            que.push_back(i);
        }
    }
    let mut dp = vec![0; n];
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                dp[v] = dp[u] + 1;
                que.push_back(v);
            }
        }
    }
    let res = dp.into_iter().max().unwrap();
    println!("{}", res);
}
