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
        n: usize, m: usize, k: usize,
        edges: [(Usize1, Usize1); m],
        phv: [(Usize1, i64); k],
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut res = BTreeSet::new();
    let mut heap = BinaryHeap::new();
    let mut max_h = vec![-1_i64; n];
    for (p, h) in phv {
        heap.push((h, p));
        max_h[p] = h;
    }
    while let Some((h, u)) = heap.pop() {
        if max_h[u] > h {
            continue;
        }
        res.insert(u + 1);
        if h == 0 {
            continue;
        }
        let new_h = h - 1;
        for &v in &graph[u] {
            if max_h[v] >= new_h {
                continue;
            }
            max_h[v] = new_h;
            heap.push((new_h, v));
        }
    }
    println!("{}", res.len());
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
