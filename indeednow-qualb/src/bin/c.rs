#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

use std::cmp::Reverse as Rev;

fn solve() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    };
    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut res = Vec::new();
    let mut heap = BinaryHeap::new();
    heap.push(Rev((0, 0)));
    while let Some(Rev((u, parent))) = heap.pop() {
        res.push(u + 1);
        for &v in &graph[u] {
            if v == parent {
                continue;
            }
            heap.push(Rev((v, u)));
        }
    }
    let res = res.into_iter().join(" ");
    println!("{}", res);
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
