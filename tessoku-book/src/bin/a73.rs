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
pub struct Graph<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    graph: Vec<Vec<(usize, Cost, bool)>>,
    vc: usize,
    inf: Cost,
}
#[derive(Clone)]
pub struct Edge<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    from: usize,
    to: usize,
    cost: Cost,
    has_tree: bool,
}
impl<Cost> From<(usize, usize, Cost, usize)> for Edge<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    fn from(e: (usize, usize, Cost, usize)) -> Edge<Cost> {
        Edge {
            from: e.0,
            to: e.1,
            cost: e.2,
            has_tree: e.3 == 1,
        }
    }
}
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    pub fn new_undirected(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost, e.has_tree));
            graph[e.to].push((e.from, e.cost, e.has_tree));
        }
        Self { graph, vc, inf }
    }
    pub fn vertex_count(&self) -> usize {
        self.vc
    }
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn shortest_path(&self, start: usize) -> Vec<Option<(Cost, usize)>> {
        let mut cost_list = vec![(self.inf, 0_usize); self.vc];
        let mut heap = std::collections::BinaryHeap::new();
        cost_list[start] = (Cost::zero(), 0);
        heap.push((std::cmp::Reverse(Cost::zero()), 0_usize, start));
        while let Some((std::cmp::Reverse(cost), tree_count, u)) = heap.pop() {
            if cost > cost_list[u].0 || (cost == cost_list[u].0 && tree_count < cost_list[u].1) {
                continue;
            }
            for &(v, w, has_tree) in &self.graph[u] {
                let new_cost = cost + w;
                let new_tree_count = tree_count + if has_tree { 1 } else { 0 };
                if new_cost < cost_list[v].0
                    || (new_cost == cost_list[v].0 && new_tree_count > cost_list[v].1)
                {
                    heap.push((std::cmp::Reverse(new_cost), new_tree_count, v));
                    cost_list[v] = (new_cost, new_tree_count);
                }
            }
        }
        cost_list
            .into_iter()
            .map(|(cost, count)| {
                if cost == self.inf {
                    None
                } else {
                    Some((cost, count))
                }
            })
            .collect::<Vec<_>>()
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, i64, usize); m]
    };

    let graph = Graph::new_undirected(edges, n, 1_i64 << 60);
    let dist = graph.shortest_path(0);
    let res = dist[n - 1].unwrap();
    println!("{} {}", res.0, res.1);
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
