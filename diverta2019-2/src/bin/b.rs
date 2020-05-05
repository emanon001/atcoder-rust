use proconio::input;
use std::collections::HashSet;

pub struct WeightedGraph {
  graph: Vec<Vec<(usize, i64)>>,
}

type WeightedEdge = (usize, usize, i64);
impl WeightedGraph {
  pub const INF: i64 = 1 << 60;

  pub fn new(edges: &[WeightedEdge], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
      graph[v].push((u, w));
    }
    Self { graph }
  }

  pub fn prim(&self, s: usize) -> i64 {
    let mut used = std::collections::HashSet::new();
    let mut heap = std::collections::BinaryHeap::new();

    let mut res = 0_i64;
    heap.push(std::cmp::Reverse((0_i64, s)));
    while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
      if used.contains(&u) {
        continue;
      }
      used.insert(u);
      res += cost;
      for &(v, c) in &self.graph[u] {
        if used.contains(&v) {
          continue;
        }
        heap.push(std::cmp::Reverse((c, v)));
      }
    }
    res
  }
}

fn main() {
  input! {
    n: usize,
    xyv: [(isize, isize); n]
  };

  if n == 1 {
    println!("1");
    std::process::exit(0);
  }

  let mut pqset = HashSet::new();
  for i in 0..n {
    for j in i + 1..n {
      let p = xyv[i].0 - xyv[j].0;
      let q = xyv[i].1 - xyv[j].1;
      pqset.insert((p, q));
    }
  }
  let mut res = 1_i64 << 60;
  for (p, q) in pqset {
    let mut edges = Vec::new();
    for i in 0..n {
      for j in i + 1..n {
        let xd = xyv[i].0 - xyv[j].0;
        let yd = xyv[i].1 - xyv[j].1;
        if p == xd && q == yd || p == -xd && q == -yd {
          edges.push((i, j, 0));
        } else {
          edges.push((i, j, 1));
        }
      }
    }
    let graph = WeightedGraph::new(&edges, n);
    let d = graph.prim(0) + 1;
    if d < res {
      res = d;
    }
  }
  println!("{}", res);
}
