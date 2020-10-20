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

#[derive(Clone)]
pub struct Graph {
    graph: Vec<Vec<(usize, f64)>>,
    vc: usize,
}
pub type Edge = (usize, usize, f64);
impl Graph {
    pub fn new(edges: Vec<Edge>, vc: usize) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for (u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vc }
    }
    pub fn new_directed(edges: Vec<Edge>, vc: usize) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for (u, v, w) in edges {
            graph[u].push((v, w));
        }
        Self { graph, vc }
    }
    pub fn add_directed_edge(&mut self, e: Edge) {
        self.graph[e.0].push((e.1, e.2));
    }
    pub fn add_edge(&mut self, e: Edge) {
        self.graph[e.0].push((e.1, e.2));
        self.graph[e.1].push((e.0, e.2));
    }
    pub fn prim(&self) -> f64 {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();
        let mut res = 0_f64;
        heap.push(std::cmp::Reverse((OrderedFloat::from(0_f64), 0)));
        while let Some(std::cmp::Reverse((weight, u))) = heap.pop() {
            if used.contains(&u) {
                continue;
            }
            used.insert(u);
            res += weight.into_inner();
            for &(v, w) in &self.graph[u] {
                if used.contains(&v) {
                    continue;
                }
                heap.push(std::cmp::Reverse((OrderedFloat::from(w), v)));
            }
        }
        res
    }
    pub fn vertex_count(&self) -> usize {
        self.vc
    }
}

fn weight(a: (f64, f64, usize), b: (f64, f64, usize)) -> f64 {
    let xd = (a.0 - b.0).abs();
    let yd = (a.1 - b.1).abs();
    let d = (xd * xd + yd * yd).sqrt();
    if a.2 == b.2 {
        d
    } else {
        d * 10.0
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        n_xycv: [(f64, f64, usize); n],
        m_xycv: [(f64, f64, usize); m],
    }

    let mut edges = Vec::new();
    for ((u, a), (v, b)) in n_xycv.iter().enumerate().tuple_combinations() {
        let w = weight(*a, *b);
        edges.push((u, v, w));
    }
    let graph = Graph::new(edges, n + m);
    let mut res = std::f64::MAX;
    for bits in 0..1 << m {
        let mut graph = graph.clone();
        let mut vertexes = Vec::new();
        for i in 0..m {
            if (bits >> i) & 1 == 1 {
                vertexes.push(i);
            }
        }
        for (&u, &v) in vertexes.iter().tuple_combinations() {
            let w = weight(m_xycv[u], m_xycv[v]);
            graph.add_edge((u + n, v + n, w));
        }
        for u in vertexes {
            for v in 0..n {
                let w = weight(m_xycv[u], n_xycv[v]);
                graph.add_edge((u + n, v, w));
            }
        }
        let cost = graph.prim();
        if cost < res {
            res = cost;
        }
    }
    println!("{}", res);
}
