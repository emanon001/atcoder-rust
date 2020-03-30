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
    let mut cost_l = vec![std::usize::MAX; self.v];
    let mut que = std::collections::VecDeque::new();
    cost_l[start] = 0;
    visited.insert(start);
    que.push_back(start);
    while let Some(u) = que.pop_front() {
      for &v in &self.graph[u] {
        if visited.contains(&v) {
          continue;
        }
        visited.insert(v);
        let new_cost = cost_l[u] + 1;
        cost_l[v] = new_cost;
        que.push_back(v);
      }
    }
    cost_l
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
    let mut cost_l = vec![inf; v];
    cost_l[s] = 0;
    let mut count = 0;
    loop {
      let mut updated = false;
      for u in 0..v {
        for &(v, w) in &self.graph[u] {
          if cost_l[u] != inf && cost_l[u] + w < cost_l[v] {
            cost_l[v] = cost_l[u] + w;
            updated = true;
          }
        }
      }
      if !updated {
        return Some(cost_l);
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
    let mut cost_l = vec![self.inf; self.v];
    let mut heap = std::collections::BinaryHeap::new();

    cost_l[start] = 0;
    heap.push(std::cmp::Reverse((0_i64, start)));

    while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
      if cost > cost_l[u] {
        continue;
      }
      for &(v, c) in &self.graph[u] {
        let new_cost = cost + c;
        if new_cost < cost_l[v] {
          heap.push(std::cmp::Reverse((new_cost, v)));
          cost_l[v] = new_cost;
        }
      }
    }
    cost_l
  }

  pub fn warshall_floyd(&self) -> Vec<Vec<i64>> {
    let inf = self.inf;
    let v = self.v;
    let mut cost = vec![vec![inf; v]; v];
    for u in 0..v {
      for &(v, w) in &self.graph[u] {
        cost[u][v] = w;
      }
    }
    for i in 0..v {
      cost[i][i] = 0;
    }
    for k in 0..v {
      for i in 0..v {
        for j in 0..v {
          cost[i][j] = std::cmp::min(cost[i][j], cost[i][k] + cost[k][j]);
        }
      }
    }
    cost
  }
}
