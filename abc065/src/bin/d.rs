use proconio::input;

pub type WeightedGraph = [Vec<(usize, i64)>];
pub fn prim(graph: &WeightedGraph, s: usize) -> i64 {
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
    for &(v, c) in &graph[u] {
      if used.contains(&v) {
        continue;
      }
      heap.push(std::cmp::Reverse((c, v)));
    }
  }
  res
}

fn main() {
  input! {
    n: usize,
    vertexes: [(usize, usize); n]
  };

  let vertexes = vertexes
    .into_iter()
    .enumerate()
    .collect::<Vec<(usize, (usize, usize))>>();
  let mut sorted_x_vertexes = vertexes.clone();
  sorted_x_vertexes.sort_by_key(|&(_, (x, _))| x);
  let mut sorted_y_vertexes = vertexes.clone();
  sorted_y_vertexes.sort_by_key(|&(_, (_, y))| y);

  let mut graph = vec![Vec::new(); n];
  for i in 0..(n - 1) {
    let (u, (x1, _)) = sorted_x_vertexes[i];
    let (v, (x2, _)) = sorted_x_vertexes[i + 1];
    let cost = (x2 - x1) as i64;
    graph[u].push((v, cost));
    graph[v].push((u, cost));
  }
  for i in 0..(n - 1) {
    let (u, (_, y1)) = sorted_y_vertexes[i];
    let (v, (_, y2)) = sorted_y_vertexes[i + 1];
    let cost = (y2 - y1) as i64;
    graph[u].push((v, cost));
    graph[v].push((u, cost));
  }

  let res = prim(&graph, 0);
  println!("{}", res);
}
