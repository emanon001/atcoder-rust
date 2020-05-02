#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct WeightedGraph {
  graph: Vec<Vec<(usize, i64, i64)>>,
  cdv: Vec<(i64, i64)>,
  v: usize,
}

type WeightedEdge = (usize, usize, i64, i64);
impl WeightedGraph {
  pub const INF: i64 = 1 << 60;
  pub const MAX_S: i64 = 50 * 50 + 5;

  pub fn new(edges: &[WeightedEdge], cdv: Vec<(i64, i64)>, v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, a, b) in edges {
      graph[u].push((v, a, b));
      graph[v].push((u, a, b));
    }
    Self { graph, cdv, v }
  }

  pub fn shortest_path(&self, start: usize, s: i64) -> Vec<Vec<i64>> {
    let mut dp = vec![vec![Self::INF; (Self::MAX_S + 5) as usize]; self.v];
    let mut heap = std::collections::BinaryHeap::new();

    let s = std::cmp::min(s, Self::MAX_S);
    dp[start][s as usize] = 0;
    // (時間, 頂点, 銀貨)
    heap.push(std::cmp::Reverse((0_i64, start, s)));
    while let Some(std::cmp::Reverse((cost, u, s))) = heap.pop() {
      if cost > dp[u][s as usize] {
        continue;
      }
      // 交換
      self.push(
        u,
        s + self.cdv[u].0,
        cost + self.cdv[u].1,
        &mut heap,
        &mut dp,
      );
      for &(v, a, b) in &self.graph[u] {
        if s < a {
          continue;
        }
        let new_s = s - a;
        let new_cost = cost + b;
        self.push(v, new_s, new_cost, &mut heap, &mut dp);
      }
    }
    dp
  }

  fn push(
    &self,
    u: usize,
    s: i64,
    t: i64,
    heap: &mut BinaryHeap<std::cmp::Reverse<(i64, usize, i64)>>,
    dp: &mut [Vec<i64>],
  ) {
    if s < 0 {
      return;
    }
    let s = std::cmp::min(s, Self::MAX_S);
    if dp[u][s as usize] <= t {
      return;
    }
    dp[u][s as usize] = t;
    heap.push(std::cmp::Reverse((t, u, s)));
  }
}

fn main() {
  input! {
    n: usize, m: usize, s: i64,
    edges: [(Usize1, Usize1, i64, i64); m],
    cdv: [(i64, i64); n]
  };

  let graph = WeightedGraph::new(&edges, cdv, n);
  let dp = graph.shortest_path(0, s);
  for i in 1..n {
    let res = dp[i].iter().min().unwrap();
    println!("{}", res);
  }
}
