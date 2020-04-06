use proconio::input;

pub struct WeightedGraph {
  graph: Vec<Vec<(usize, i64)>>,
  v: usize,
}

impl WeightedGraph {
  pub const INF: i64 = 1 << 60;

  pub fn new(edges: &[(usize, usize, i64)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
      graph[v].push((u, w));
    }
    Self { graph, v }
  }

  pub fn new_directed(edges: &[(usize, usize, i64)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
    }
    Self { graph, v }
  }

  pub fn shortest_path(&self, start: usize) -> Vec<i64> {
    let mut cost_list = vec![Self::INF; self.v];
    let mut heap = std::collections::BinaryHeap::new();

    cost_list[start] = 0;
    heap.push(std::cmp::Reverse((0_i64, start)));

    while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
      if cost > cost_list[u] {
        continue;
      }
      for &(v, c) in &self.graph[u] {
        let new_cost = cost + c;
        if new_cost < cost_list[v] {
          heap.push(std::cmp::Reverse((new_cost, v)));
          cost_list[v] = new_cost;
        }
      }
    }
    cost_list
  }
}

fn main() {
  input! {
    n: usize,
    av: [i64; n]
  };

  let mut edges = Vec::new();
  for i in 0..n {
    if i + 1 < n {
      edges.push((i, i + 1, (av[i] - av[i + 1]).abs()));
    }
    if i + 2 < n {
      edges.push((i, i + 2, (av[i] - av[i + 2]).abs()));
    }
  }
  let graph = WeightedGraph::new_directed(&edges, n);
  let res = graph.shortest_path(0)[n - 1];
  println!("{}", res);
}
