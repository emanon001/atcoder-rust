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

  pub fn solve(&self, start: usize) -> Vec<Option<usize>> {
    let mut visited = std::collections::HashSet::new();
    let mut parent_list = vec![None; self.v];
    let mut que = std::collections::VecDeque::new();
    visited.insert(start);
    que.push_back(start);
    while let Some(u) = que.pop_front() {
      for &v in &self.graph[u] {
        if visited.contains(&v) {
          continue;
        }
        visited.insert(v);
        parent_list[v] = Some(u);
        que.push_back(v);
      }
    }
    parent_list
  }
}

fn main() {
  input! {
    n: usize, m: usize,
    edges: [(Usize1, Usize1); m]
  };

  let graph = Graph::new(&edges, n);
  let res = graph.solve(0);
  println!("Yes");
  for i in 1..n {
    println!("{}", res[i].unwrap() + 1);
  }
}
