#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

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
        cost.into_iter()
            .map(|v| self.optionalize(v))
            .collect::<Vec<_>>()
    }

    fn optionalize(&self, v: Vec<i64>) -> Vec<Option<i64>> {
        v.into_iter()
            .map(|x| if x == Self::INF { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

fn solve() {
    input! {
        n: usize, m: usize, l: i64,
        edges: [(Usize1, Usize1, i64); m],
        q: usize,
        queries: [(Usize1, Usize1); q]
    };

    let graph = WeightedGraph::new(&edges, n);
    let d = graph.warshall_floyd();
    let edges = (0..n)
        .combinations(2)
        .flat_map(|v| {
            let u = v[0];
            let v = v[1];
            if let Some(c) = d[u][v] {
                if c <= l {
                    return Some((u, v, 1));
                }
            }
            None
        })
        .collect::<Vec<_>>();
    let graph = WeightedGraph::new(&edges, n);
    let d = graph.warshall_floyd();
    for (s, t) in queries {
        let res = if let Some(c) = d[s][t] { c - 1 } else { -1 };
        println!("{}", res);
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
