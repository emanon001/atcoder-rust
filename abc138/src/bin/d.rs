#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct Graph {
    graph: Vec<Vec<usize>>,
    v: usize,
}

type Edge = (usize, usize);
impl Graph {
    pub fn new(edges: &[Edge], v: usize) -> Self {
        let mut graph = vec![Vec::new(); v];
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        Self { graph, v }
    }

    pub fn shortest_path(&self, start: usize, pv: &[usize]) -> Vec<Option<usize>> {
        let mut cost_list = vec![None; self.v];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(pv[start]);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &v in &self.graph[u] {
                if cost_list[v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + pv[v];
                cost_list[v] = Some(new_cost);
                que.push_back(v);
            }
        }
        cost_list
    }
}

fn main() {
    input! {
        n: usize, q: usize,
        edges: [(Usize1, Usize1); n - 1],
        queries: [(Usize1, usize); q]
    };

    let mut pv = vec![0; n];
    for (p, x) in queries {
        pv[p] += x;
    }
    let graph = Graph::new(&edges, n);
    let res = graph.shortest_path(0, &pv);
    println!("{}", res.into_iter().map(|o| o.unwrap()).join(" "));
}
