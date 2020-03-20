use proconio::input;
use proconio::marker::Usize1;

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

fn main() {
  input! {
    n: usize, k: usize
  };
  let mut graph: Graph = vec![Vec::new(); n];
  for _ in 0..k {
    input! { kind: usize }
    match kind {
      0 => {
        input! { a: Usize1, b: Usize1 }
        let d = shortest_path(&graph, a)[b];
        if d == std::i64::MAX {
          println!("-1");
        } else {
          println!("{}", d);
        };
      }
      1 => {
        input! { c: Usize1, d: Usize1, e: i64 };
        graph[c].push((d, e));
        graph[d].push((c, e));
      }
      _ => {}
    };
  }
}
