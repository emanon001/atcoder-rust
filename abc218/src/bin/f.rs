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
pub struct Graph {
    graph: Vec<HashSet<(usize, i64)>>,
    vc: usize,
    inf: i64,
}
#[derive(Clone)]
pub struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}
impl From<(usize, usize)> for Edge {
    fn from(e: (usize, usize)) -> Edge {
        Edge {
            from: e.0,
            to: e.1,
            cost: 0,
        }
    }
}
impl Graph {
    pub fn new_directed(edges: Vec<impl Into<Edge>>, vc: usize, inf: i64) -> Self {
        let mut graph = vec![HashSet::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].insert((e.to, e.cost));
        }
        Self { graph, vc, inf }
    }
    pub fn add_directed_edge(&mut self, e: impl Into<Edge>) {
        let e = e.into();
        self.graph[e.from].insert((e.to, e.cost));
    }
    pub fn remove_directed_edge(&mut self, e: impl Into<Edge>) {
        let e = e.into();
        self.graph[e.from].remove(&(e.to, e.cost));
    }
}

impl Graph {
    pub fn shortest_path_1(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![None; self.vc];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for (v, _) in &self.graph[u] {
                if cost_list[*v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + 1;
                cost_list[*v] = Some(new_cost);
                que.push_back(*v);
            }
        }
        cost_list
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };
    let mut graph = Graph::new_directed(edges.clone(), n, 1_i64 << 60);
    for e in edges {
        graph.remove_directed_edge(e);
        let res = graph.shortest_path_1(0)[n - 1].unwrap_or(-1);
        println!("{}", res);
        graph.add_directed_edge(e);
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
