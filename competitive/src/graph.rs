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

pub struct WeightedGraph {
  graph: Vec<Vec<(usize, i64)>>,
  v: usize,
  inf: i64,
}

impl WeightedGraph {
  pub fn new(edges: &[(usize, usize, i64)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
      graph[v].push((u, w));
    }
    let inf = 1 << 60;
    Self { graph, v, inf }
  }

  pub fn new_directed(edges: &[(usize, usize, i64)], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
    }
    let inf = 1 << 60;
    Self { graph, v, inf }
  }

  pub fn bellman_ford(&self, s: usize) -> Option<Vec<i64>> {
    let v = self.v;
    let inf = self.inf;
    let mut d = vec![inf; v];
    d[s] = 0;
    let mut count = 0;
    loop {
      let mut updated = false;
      for u in 0..v {
        for &(v, w) in &self.graph[u] {
          if d[u] != inf && d[u] + w < d[v] {
            d[v] = d[u] + w;
            updated = true;
          }
        }
      }
      if !updated {
        return Some(d);
      }
      count += 1;
      if count == v {
        return None;
      }
    }
  }

  pub fn inf(&self) -> i64 {
    self.inf
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

  pub fn reachable_vertexes(&self, s: usize) -> std::collections::HashSet<usize> {
    let mut visited = std::collections::HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    visited.insert(s);
    while let Some(u) = queue.pop_front() {
      for &(v, _) in &self.graph[u] {
        if visited.contains(&v) {
          continue;
        }
        visited.insert(v);
        queue.push_back(v);
      }
    }
    visited
  }

  pub fn shortest_path(&self, start: usize) -> Vec<i64> {
    let mut dist = vec![self.inf; self.v];
    let mut heap = std::collections::BinaryHeap::new();

    dist[start] = 0;
    heap.push(std::cmp::Reverse((0_i64, start)));

    while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
      if cost > dist[u] {
        continue;
      }
      for &(v, c) in &self.graph[u] {
        let new_cost = cost + c;
        if new_cost < dist[v] {
          heap.push(std::cmp::Reverse((new_cost, v)));
          dist[v] = new_cost;
        }
      }
    }
    dist
  }

  pub fn warshall_floyd(&self) -> Vec<Vec<i64>> {
    let inf = self.inf;
    let v = self.v;
    let mut d = vec![vec![inf; v]; v];
    for u in 0..v {
      for &(v, w) in &self.graph[u] {
        d[u][v] = w;
      }
    }
    for i in 0..v {
      d[i][i] = 0;
    }
    for k in 0..v {
      for i in 0..v {
        for j in 0..v {
          d[i][j] = std::cmp::min(d[i][j], d[i][k] + d[k][j]);
        }
      }
    }
    d
  }
}
