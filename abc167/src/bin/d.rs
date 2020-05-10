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
  pub const INF: usize = std::usize::MAX;

  pub fn new(edges: &[Edge], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v) in edges {
      graph[u].push(v);
      graph[v].push(u);
    }
    Self { graph, v }
  }

  pub fn new_directed(edges: &[Edge], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v) in edges {
      graph[u].push(v);
    }
    Self { graph, v }
  }

  pub fn add_directed_edge(&mut self, e: Edge) {
    self.graph[e.0].push(e.1);
  }

  pub fn add_edge(&mut self, e: Edge) {
    self.graph[e.0].push(e.1);
    self.graph[e.1].push(e.0);
  }

  pub fn shortest_path(&self, start: usize) -> Vec<usize> {
    let mut visited = std::collections::HashSet::new();
    let mut cost_list = vec![Self::INF; self.v];
    let mut que = std::collections::VecDeque::new();
    cost_list[start] = 0;
    visited.insert(start);
    que.push_back(start);
    while let Some(u) = que.pop_front() {
      for &v in &self.graph[u] {
        if visited.contains(&v) {
          continue;
        }
        visited.insert(v);
        let new_cost = cost_list[u] + 1;
        cost_list[v] = new_cost;
        que.push_back(v);
      }
    }
    cost_list
  }
}

fn main() {
  input! {
    n: usize, k: usize,
    av: [Usize1; n]
  };

  let mut edges = Vec::new();
  for i in 0..n {
    let u = i;
    let v = av[i];
    edges.push((u, v));
  }
  let graph = Graph::new_directed(&edges, n);
  let d = graph.shortest_path(0);
  let mut max = 0;
  let mut max_pos = 0;
  for i in 0..n {
    if d[i] < Graph::INF && d[i] > max {
      max = d[i];
      max_pos = i;
    }
  }
  if k <= max {
    let res = d.iter().position(|&x| x == k).unwrap() + 1;
    println!("{}", res);
    std::process::exit(0);
  }

  let rest = k - (max + 1);
  let p = max - d[av[max_pos]] + 1;
  let pos = rest % p + d[av[max_pos]];
  let res = d.iter().position(|&x| x == pos).unwrap() + 1;
  println!("{}", res);
}
