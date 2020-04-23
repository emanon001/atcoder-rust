#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

const INF: u64 = 1 << 60;

fn shortest_path(graph: &[Vec<(usize, u64)>], start: usize) -> Vec<u64> {
  let mut heap = std::collections::BinaryHeap::new();

  let mut cost_list = vec![INF; graph.len()];
  cost_list[start] = 0;
  heap.push(std::cmp::Reverse((0_u64, start)));

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
    n: usize,
    table: [[u64; n]; n]
  };

  let mut all_edges = Vec::new();
  for i in 0..n {
    for j in 0..n {
      if i == j {
        continue;
      }
      if i > j {
        continue;
      }
      all_edges.push((i, j, table[i][j]));
    }
  }
  all_edges.sort_by_key(|e| e.2);
  let mut graph = vec![Vec::new(); n];
  let mut res = 0;
  for e in all_edges {
    let cost_list = shortest_path(&graph, e.0);
    // println!("{:?}, {:?}", e, d);
    if cost_list[e.1] < e.2 {
      println!("-1");
      std::process::exit(0);
    }
    if cost_list[e.1] > e.2 {
      graph[e.0].push((e.1, e.2));
      graph[e.1].push((e.0, e.2));
      res += e.2;
    }
  }
  println!("{}", res);
}
