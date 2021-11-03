#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Clone)]
pub struct Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    graph: Vec<Vec<(usize, Cost, i64)>>,
    vc: usize,
    inf: Cost,
}
#[derive(Clone)]
pub struct Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    from: usize,
    to: usize,
    cost: Cost,
    t: i64,
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn new(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost, e.t));
        }
        Self { graph, vc, inf }
    }
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn shortest_path(&self, start: usize, init_t: i64) -> Vec<(Cost, i64)> {
        let mut cost_list = vec![(self.inf, 0); self.vc];
        let mut heap = std::collections::BinaryHeap::new();
        // (コスト, 満足度)
        cost_list[start] = (Cost::zero(), init_t);
        heap.push((Reverse(Cost::zero()), init_t, start));
        while let Some((Reverse(cost), t, u)) = heap.pop() {
            if cost > cost_list[u].0 || t < cost_list[u].1 {
                continue;
            }
            for &(v, w, t2) in &self.graph[u] {
                let new_cost = cost + w;
                let new_t = t + t2;
                if new_cost < cost_list[v].0
                    || (new_cost == cost_list[v].0 && new_t > cost_list[v].1)
                {
                    heap.push((Reverse(new_cost), new_t, v));
                    cost_list[v] = (new_cost, new_t);
                }
            }
        }
        cost_list
    }
}

impl<Cost> From<(usize, usize, Cost, i64)> for Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    fn from(e: (usize, usize, Cost, i64)) -> Edge<Cost> {
        Edge {
            from: e.0,
            to: e.1,
            cost: e.2,
            t: e.3,
        }
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        av: [i64; n],
        edges: [(Usize1, Usize1, i64); m]
    };

    let mut new_edges = Vec::new();
    for (u, v, t) in edges {
        new_edges.push((u, v, t, av[v]));
        new_edges.push((v, u, t, av[u]));
    }
    let g = Graph::new(new_edges, n, 1_i64 << 60);
    let d = g.shortest_path(0, av[0]);
    let res = d[n - 1].1;
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
