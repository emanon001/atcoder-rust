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
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![Vec::new(); n];
    let mut indegree = vec![0; n];
    for (u, v) in edges {
        indegree[v] += 1;
        graph[u].push(v);
    }
    let mut heap = BinaryHeap::new();
    for i in 0..n {
        if indegree[i] == 0 {
            heap.push(std::cmp::Reverse(i));
        }
    }
    let mut res = Vec::new();
    while let Some(std::cmp::Reverse(u)) = heap.pop() {
        for &v in &graph[u] {
            indegree[v] -= 1;
            if indegree[v] == 0 {
                heap.push(std::cmp::Reverse(v));
            }
        }
        res.push(u + 1);
    }
    if res.len() < n {
        println!("-1");
        return;
    }
    println!("{}", res.into_iter().join(" "));
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
