#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn prim(graph: &[Vec<(usize, i64, usize)>]) -> Vec<usize> {
    let mut used = std::collections::HashSet::new();
    let mut heap = std::collections::BinaryHeap::new();
    let mut res = Vec::new();
    let inf = 1 << 30;
    heap.push((std::cmp::Reverse(0_i64), 0, inf));
    while let Some((std::cmp::Reverse(w1), u, r1)) = heap.pop() {
        if used.contains(&u) {
            continue;
        }
        used.insert(u);
        if r1 < inf {
            res.push(r1);
        }
        for &(v, w, r2) in &graph[u] {
            if used.contains(&v) {
                continue;
            }
            heap.push((std::cmp::Reverse(w1 + w), v, r2));
        }
    }
    res
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, i64); m]
    };

    let mut graph = vec![Vec::new(); n];
    for (i, (u, v, w)) in edges.into_iter().enumerate() {
        graph[u].push((v, w, i + 1));
        graph[v].push((u, w, i + 1));
    }
    let res = prim(&graph);
    println!("{}", res.iter().join(" "));
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
