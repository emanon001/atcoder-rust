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
        n: usize, m: usize, s: Usize1,
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut heap = BinaryHeap::new();
    heap.push((s, s));
    let mut dp = vec![-1_isize; n];
    dp[s] = s as isize;
    while let Some((min_v, u)) = heap.pop() {
        if dp[u] > min_v as isize {
            continue;
        }
        for &v in &graph[u] {
            let min_v = min_v.min(u);
            if min_v as isize > dp[v] {
                dp[v] = min_v as isize;
                heap.push((min_v, v));
            }
        }
    }
    for u in 0..n {
        if (u as isize) <= dp[u] {
            println!("{}", u + 1);
        }
    }
}
