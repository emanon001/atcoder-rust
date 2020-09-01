#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Clone)]
pub struct WeightedGraph {
    graph: Vec<Vec<(usize, i64)>>,
    vn: usize,
}
pub type WeightedEdge = (usize, usize, i64);
impl WeightedGraph {
    const INF: i64 = 1 << 60;
    pub fn new(edges: &[WeightedEdge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vn }
    }
    pub fn warshall_floyd(&self) -> Vec<Vec<i64>> {
        let inf = Self::INF;
        let vn = self.vn;
        let mut cost_list = vec![vec![inf; vn]; vn];
        for u in 0..vn {
            for &(v, w) in &self.graph[u] {
                cost_list[u][v] = w;
            }
        }
        for i in 0..vn {
            cost_list[i][i] = 0;
        }
        for k in 0..vn {
            for i in 0..vn {
                for j in 0..vn {
                    cost_list[i][j] =
                        std::cmp::min(cost_list[i][j], cost_list[i][k] + cost_list[k][j]);
                }
            }
        }
        cost_list
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, i64); m],
        k: usize,
        xyzv: [(Usize1, Usize1, i64); k]
    };

    let graph = WeightedGraph::new(&edges, n);
    let mut dist = graph.warshall_floyd();
    for (x, y, z) in xyzv {
        let mut sum = 0;
        for u in 0..n - 1 {
            for v in u + 1..n {
                let min = dist[u][v]
                    .min(dist[u][x] + dist[y][v] + z)
                    .min(dist[u][y] + dist[x][v] + z);
                sum += min;
                dist[u][v] = min;
                dist[v][u] = min;
            }
        }
        println!("{}", sum);
    }
}
