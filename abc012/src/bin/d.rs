use proconio::input;
use proconio::marker::Usize1;

pub type WeightedGraph = [Vec<(usize, i64)>];
fn warshall_floyd(graph: &WeightedGraph) -> Vec<Vec<i64>> {
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

fn main() {
  input! {
    n: usize, m: usize,
    edges: [(Usize1, Usize1, i64); m]
  };

  let mut graph = vec![Vec::new(); n];
  for (u, v, w) in edges {
    graph[u].push((v, w));
    graph[v].push((u, w));
  }

  let d = warshall_floyd(&graph);
  let mut res = std::i64::MAX;
  for v in d {
    let n = v.into_iter().max().unwrap();
    if n < res {
      res = n;
    }
  }
  println!("{}", res);
}
