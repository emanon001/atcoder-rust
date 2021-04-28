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
    graph: Vec<Vec<(usize, i64)>>,
    vc: usize,
    inf: i64,
}
pub type Edge = (usize, usize, i64);
impl Graph
{
    pub fn new(edges: Vec<Edge>, vc: usize, inf: i64) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for (u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vc, inf }
    }
    pub fn shortest_path(&self, start: usize, goal: usize, _mod: usize) -> Vec<Vec<i64>> {
        let mut cost_list = vec![vec![self.inf; _mod]; self.vc];
        let mut heap = std::collections::BinaryHeap::new();
        cost_list[start][0] = 0;
        heap.push(std::cmp::Reverse((0, start)));
        while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
            if u == goal {
                continue;
            }
            if cost > cost_list[u][(cost % _mod as i64) as usize] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                let new_cost = cost + w;
                let m = (new_cost % _mod as i64) as usize;
                if new_cost < cost_list[v][m] {
                    heap.push(std::cmp::Reverse((new_cost, v)));
                    cost_list[v][m] = new_cost;
                }
            }
        }
        cost_list
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(usize, usize, i64); m]
    };

    let graph = Graph::new(edges, n, 1_i64 << 60);
    let a = graph.shortest_path(0, n - 1, 4)[n - 1][0];
    let b = graph.shortest_path(0, n - 1, 7)[n - 1][0];
    let res = a.min(b);
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
