pub type Graph = [Vec<usize>];
pub type WeightedGraph = [Vec<(usize, i64)>];

pub fn bellman_ford(graph: &WeightedGraph, s: usize) -> Option<Vec<i64>> {
  let inf = 1_i64 << 60;
  let v = graph.len();
  let mut d = vec![inf; v];
  d[s] = 0;
  let mut count = 0;
  loop {
    let mut updated = false;
    for u in 0..v {
      for &(v, w) in &graph[u] {
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

pub fn reachable_vertexes(graph: &WeightedGraph, s: usize) -> std::collections::HashSet<usize> {
  let mut visited = std::collections::HashSet::new();
  let mut queue = std::collections::VecDeque::new();
  visited.insert(s);
  while let Some(u) = queue.pop_front() {
    for &(v, _) in &graph[u] {
      if visited.contains(&v) {
        continue;
      }
      visited.insert(v);
      queue.push_back(v);
    }
  }
  visited
}

pub fn shortest_path(graph: &WeightedGraph, start: usize) -> Vec<i64> {
  let mut dist = vec![std::i64::MAX; graph.len()];
  let mut heap = std::collections::BinaryHeap::new();

  dist[start] = 0;
  heap.push(std::cmp::Reverse((0_i64, start)));

  while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
    if cost > dist[u] {
      continue;
    }
    for &(v, c) in &graph[u] {
      let new_cost = cost + c;
      if new_cost < dist[v] {
        heap.push(std::cmp::Reverse((new_cost, v)));
        dist[v] = new_cost;
      }
    }
  }
  dist
}

pub fn warshall_floyd(graph: &WeightedGraph) -> Vec<Vec<i64>> {
  let inf = 1_i64 << 60;
  let v = graph.len();
  let mut d = vec![vec![inf; v]; v];
  for u in 0..v {
    for &(v, w) in &graph[u] {
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
