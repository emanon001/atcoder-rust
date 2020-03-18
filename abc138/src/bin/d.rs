use proconio::input;
use proconio::marker::Usize1;

fn calc_counters(u: usize, parent: usize, graph: &Vec<Vec<usize>>, counters: &mut Vec<u32>) {
  let c = counters[u];
  for &v in &graph[u] {
    if v == parent {
      continue;
    }
    counters[v] += c;
    calc_counters(v, u, graph, counters);
  }
}

fn main() {
  input! {
    n: usize, q: usize,
    edges: [(Usize1, Usize1); n - 1],
    queries: [(Usize1, u32); q],
  };

  let mut graph = Vec::new();
  for _ in 0..n {
    graph.push(Vec::new());
  }
  for (u, v) in edges {
    graph[u].push(v);
    graph[v].push(u);
  }
  let mut counters = vec![0; n];
  for (u, x) in queries {
    counters[u] += x;
  }
  calc_counters(0, n, &mut graph, &mut counters);
  let res = counters
    .into_iter()
    .map(|n| n.to_string())
    .collect::<Vec<String>>()
    .join(" ");
  println!("{}", res);
}
