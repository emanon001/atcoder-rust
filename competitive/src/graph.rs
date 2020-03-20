pub type Graph = Vec<Vec<(usize, i64)>>;

pub fn shortest_path(graph: &Graph, start: usize) -> Vec<i64> {
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
