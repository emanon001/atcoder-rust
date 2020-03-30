use proconio::input;

pub struct Graph {
  graph: Vec<Vec<usize>>,
  v: usize,
}

impl Graph {
  pub fn new(edges: &[(usize, usize)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v) in edges {
      graph[u].push(v);
      graph[v].push(u);
    }
    Self { graph, v }
  }

  pub fn new_directed(edges: &[(usize, usize)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v) in edges {
      graph[u].push(v);
    }
    Self { graph, v }
  }

  pub fn shortest_path(&self, start: usize) -> Vec<usize> {
    let mut visited = std::collections::HashSet::new();
    let mut d = vec![std::usize::MAX; self.v];
    let mut que = std::collections::VecDeque::new();
    d[start] = 0;
    visited.insert(start);
    que.push_back(start);
    while let Some(u) = que.pop_front() {
      for &v in &self.graph[u] {
        if visited.contains(&v) {
          continue;
        }
        visited.insert(v);
        let new_cost = d[u] + 1;
        d[v] = new_cost;
        que.push_back(v);
      }
    }
    d
  }
}

fn main() {
  input! {
    n: usize, x: usize, y: usize
  };

  let mut edges = Vec::new();
  for i in 0..(n - 1) {
    edges.push((i, i + 1));
  }
  edges.push((x - 1, y - 1));

  let graph = Graph::new(&edges, n);
  let mut table = std::collections::HashMap::new();
  for i in 0..n {
    for c in graph.shortest_path(i) {
      let x = table.entry(c).or_insert(0);
      *x += 1;
    }
  }
  for i in 1..n {
    let x = table.get(&i).unwrap_or(&0) / 2;
    println!("{}", x);
  }
}
