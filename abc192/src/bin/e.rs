#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Clone)]
pub struct Graph
{
    graph: Vec<Vec<(usize, i64, i64)>>,
    vc: usize,
    inf: i64,
}
pub type Edge = (usize, usize, i64, i64);
impl Graph {
    pub fn new(edges: Vec<Edge>, vc: usize, inf: i64) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for (u, v, w, k) in edges {
            graph[u].push((v, w, k));
            graph[v].push((u, w, k));
        }
        Self { graph, vc, inf }
    }
    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![self.inf; self.vc];
        let mut heap = std::collections::BinaryHeap::new();
        cost_list[start] = 0;
        heap.push(std::cmp::Reverse((0, start)));
        while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w, k) in &self.graph[u] {
                let new_cost = cost + if cost % k == 0 { w } else { k - (cost % k) + w };
                if new_cost < cost_list[v] {
                    heap.push(std::cmp::Reverse((new_cost, v)));
                    cost_list[v] = new_cost;
                }
            }
        }
        self.optionalize(cost_list)
    }
    pub fn vertex_count(&self) -> usize {
        self.vc
    }
    fn optionalize(&self, v: Vec<i64>) -> Vec<Option<i64>> {
        v.into_iter()
            .map(|x| if x == self.inf { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

fn solve() {
    input! {
        n: usize, m: usize, x: Usize1, y: Usize1,
        edges: [(Usize1, Usize1, i64, i64); m]
    };

    let graph = Graph::new(edges, n, 1_i64 << 60);
    let res = graph.shortest_path(x);
    if let Some(res) = res[y] {
        println!("{}", res);
    } else {
        println!("-1");
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
