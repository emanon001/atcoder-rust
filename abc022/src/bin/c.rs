use proconio::input;
use proconio::marker::Usize1;

const INF: i64 = 1 << 60;

fn shortest_path(graph: &Vec<std::collections::VecDeque<(usize, i64)>>, start: usize) -> Vec<i64> {
  let mut cost_list = vec![INF; graph.len()];
  let mut heap = std::collections::BinaryHeap::new();

  cost_list[start] = 0;
  heap.push(std::cmp::Reverse((0_i64, start)));

  while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
    if cost > cost_list[u] {
      continue;
    }
    for &(v, c) in &graph[u] {
      let new_cost = cost + c;
      if new_cost < cost_list[v] {
        heap.push(std::cmp::Reverse((new_cost, v)));
        cost_list[v] = new_cost;
      }
    }
  }
  cost_list
}

fn main() {
  input! {
    n: usize, m: usize,
    edges: [(Usize1, Usize1, i64); m]
  };

  let mut graph = vec![std::collections::VecDeque::new(); n];
  for (u, v, w) in edges {
    graph[u].push_back((v, w));
    graph[v].push_back((u, w));
  }
  let mut res = INF;
  for (u, w) in graph[0].clone() {
    graph[0].pop_front();
    let cost = shortest_path(&graph, 0);
    if cost[u] + w < res {
      res = cost[u] + w;
    }
    graph[0].push_back((u, w));
  }
  if res == INF {
    println!("-1");
  } else {
    println!("{}", res);
  }
}
