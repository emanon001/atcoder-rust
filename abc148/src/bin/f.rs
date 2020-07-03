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
    vn: usize,
}

type Edge = (usize, usize);
impl Graph {
    pub fn new(edges: &[Edge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        Self { graph, vn }
    }

    pub fn new_directed(edges: &[Edge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v) in edges {
            graph[u].push(v);
        }
        Self { graph, vn }
    }

    pub fn add_directed_edge(&mut self, e: Edge) {
        self.graph[e.0].push(e.1);
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.graph[e.0].push(e.1);
        self.graph[e.1].push(e.0);
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<usize>> {
        let mut cost_list = vec![None; self.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &v in &self.graph[u] {
                if cost_list[v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + 1;
                cost_list[v] = Some(new_cost);
                que.push_back(v);
            }
        }
        cost_list
    }
}

fn solve() {
    input! {
        n: usize, u: Usize1, v: Usize1,
        edges: [(Usize1, Usize1); n - 1]
    };

    let graph = Graph::new(&edges, n);
    let td = graph.shortest_path(u);
    let ad = graph.shortest_path(v);
    let mut res = 0;
    for i in 0..n {
        let t = td[i].unwrap();
        let a = ad[i].unwrap();
        if t < a {
            res = res.max(a - 1);
        }
    }
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
