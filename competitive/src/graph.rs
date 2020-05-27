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

  pub fn shortest_path(&self, start: usize) -> Vec<Option<usize>> {
    let mut cost_list = vec![None; self.v];
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

pub struct WeightedGraph {
  graph: Vec<Vec<(usize, i64)>>,
  v: usize,
}

type WeightedEdge = (usize, usize, i64);
impl WeightedGraph {
  const INF: i64 = 1 << 60;

  pub fn new(edges: &[WeightedEdge], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
      graph[v].push((u, w));
    }
    Self { graph, v }
  }

  pub fn new_directed(edges: &[WeightedEdge], v: usize) -> Self {
    let mut graph = vec![Vec::new(); v];
    for &(u, v, w) in edges {
      graph[u].push((v, w));
    }
    Self { graph, v }
  }

  pub fn add_directed_edge(&mut self, e: WeightedEdge) {
    self.graph[e.0].push((e.1, e.2));
  }

  pub fn add_edge(&mut self, e: WeightedEdge) {
    self.graph[e.0].push((e.1, e.2));
    self.graph[e.1].push((e.0, e.2));
  }

  pub fn bellman_ford(&self, s: usize) -> Option<Vec<Option<i64>>> {
    let v = self.v;
    let inf = Self::INF;
    let mut cost_list = vec![inf; v];
    cost_list[s] = 0;
    let mut count = 0;
    loop {
      let mut updated = false;
      for u in 0..v {
        for &(v, w) in &self.graph[u] {
          if cost_list[u] != inf && cost_list[u] + w < cost_list[v] {
            cost_list[v] = cost_list[u] + w;
            updated = true;
          }
        }
      }
      if !updated {
        return Some(self.optionalize(cost_list));
      }
      count += 1;
      if count == v {
        return None;
      }
    }
  }

  pub fn prim(&self) -> i64 {
    let mut used = std::collections::HashSet::new();
    let mut heap = std::collections::BinaryHeap::new();

    let mut res = 0_i64;
    heap.push(std::cmp::Reverse((0_i64, 0)));
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

  pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
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
    self.optionalize(cost_list)
  }

  pub fn warshall_floyd(&self) -> Vec<Vec<Option<i64>>> {
    let inf = Self::INF;
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
      .into_iter()
      .map(|v| self.optionalize(v))
      .collect::<Vec<_>>()
  }

  fn optionalize(&self, v: Vec<i64>) -> Vec<Option<i64>> {
    v.into_iter()
      .map(|x| if x == Self::INF { None } else { Some(x) })
      .collect::<Vec<_>>()
  }
}

#[cfg(test)]
mod tests {
  mod graph {
    use super::super::Graph;
    #[test]
    fn shortest_path() {
      let edges = vec![(0, 1), (0, 2), (1, 2), (2, 3), (2, 4), (3, 4), (4, 5)];
      // 頂点6には到達しない
      let graph = Graph::new(&edges, 7);
      let res = graph.shortest_path(0);
      assert_eq!(Some(0), res[0]);
      assert_eq!(Some(1), res[1]);
      assert_eq!(Some(1), res[2]);
      assert_eq!(Some(2), res[3]);
      assert_eq!(Some(2), res[4]);
      assert_eq!(Some(3), res[5]);
      assert_eq!(None, res[6]);
    }
  }
}
