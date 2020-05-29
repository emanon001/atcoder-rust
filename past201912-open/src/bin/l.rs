#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use ordered_float::OrderedFloat;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct WeightedGraph {
    graph: Vec<Vec<(usize, f64)>>,
}

type WeightedEdge = (usize, usize, f64);
impl WeightedGraph {
    pub fn new(edges: &[WeightedEdge], v: usize) -> Self {
        let mut graph = vec![Vec::new(); v];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph }
    }

    pub fn new_directed(edges: &[WeightedEdge], v: usize) -> Self {
        let mut graph = vec![Vec::new(); v];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
        }
        Self { graph }
    }

    pub fn prim(&self) -> f64 {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();

        let mut res = 0_f64;
        heap.push(std::cmp::Reverse((OrderedFloat::from(0_f64), 0)));
        while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
            if used.contains(&u) {
                continue;
            }
            used.insert(u);
            res += cost.into_inner();
            for &(v, c) in &self.graph[u] {
                if used.contains(&v) {
                    continue;
                }
                heap.push(std::cmp::Reverse((OrderedFloat::from(c), v)));
            }
        }
        res
    }
}

fn distance(v1: (usize, usize), v2: (usize, usize)) -> f64 {
    let xd = ((v1.0 as isize) - (v2.0 as isize)).abs() as usize;
    let yd = ((v1.1 as isize) - (v2.1 as isize)).abs() as usize;
    ((xd * xd + yd * yd) as f64).sqrt()
}

fn main() {
    input! {
        n: usize, m: usize,
        nv: [(usize, usize, usize); n],
        mv: [(usize, usize, usize); m]
    };

    let mut res = std::f64::MAX;
    let mut edges = Vec::new();
    for u in 0..n - 1 {
        let (x1, y1, c1) = nv[u];
        for v in u + 1..n {
            let (x2, y2, c2) = nv[v];
            let d = distance((x1, y1), (x2, y2));
            let cost = if c1 == c2 { d } else { d * 10.0 };
            edges.push((u, v, cost));
        }
    }
    for bit in 0..1 << m {
        let mut edges = edges.clone();
        for u in 0..m {
            if (bit >> u) & 1 == 0 {
                continue;
            }
            let (x1, y1, c1) = mv[u];
            for v in 0..n {
                let (x2, y2, c2) = nv[v];
                let d = distance((x1, y1), (x2, y2));
                let cost = if c1 == c2 { d } else { d * 10.0 };
                edges.push((u + n, v, cost));
            }
        }
        for u in 0..m {
            if (bit >> u) & 1 == 0 {
                continue;
            }
            let (x1, y1, c1) = mv[u];
            for v in 0..m {
                if (bit >> v) & 1 == 0 {
                    continue;
                }
                let (x2, y2, c2) = mv[v];
                let d = distance((x1, y1), (x2, y2));
                let cost = if c1 == c2 { d } else { d * 10.0 };
                edges.push((u + n, v + n, cost));
            }
        }
        let graph = WeightedGraph::new(&edges, n + m);
        let d = graph.prim();
        if d < res {
            res = d;
        }
    }
    println!("{}", res);
}
