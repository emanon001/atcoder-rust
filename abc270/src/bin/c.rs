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
        n: usize, x: Usize1, y: Usize1,
        edges: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut parents = vec![n; n];
    let mut que = VecDeque::new();
    que.push_back((x, n));
    while let Some((u, p)) = que.pop_front() {
        for &v in &graph[u] {
            if v == p {
                continue;
            }
            parents[v] = u;
            que.push_back((v, u));
        }
    }
    let mut res = VecDeque::new();
    res.push_back(y);
    let mut v = y;
    while v != x {
        v = parents[v];
        res.push_front(v);
    }
    println!("{}", res.iter().map(|u| u + 1).join(" "));
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
