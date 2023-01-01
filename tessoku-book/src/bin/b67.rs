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
pub struct Graph<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    graph: Vec<Vec<(usize, Cost)>>,
    vc: usize,
    inf: Cost,
}
#[derive(Clone)]
pub struct Edge<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    from: usize,
    to: usize,
    cost: Cost,
}
impl<Cost> From<(usize, usize, Cost)> for Edge<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    fn from(e: (usize, usize, Cost)) -> Edge<Cost> {
        Edge {
            from: e.0,
            to: e.1,
            cost: e.2,
        }
    }
}
impl<Cost> From<(usize, usize)> for Edge<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    fn from(e: (usize, usize)) -> Edge<Cost> {
        Edge {
            from: e.0,
            to: e.1,
            cost: Cost::one(),
        }
    }
}
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    pub fn new_undirected(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost));
            graph[e.to].push((e.from, e.cost));
        }
        Self { graph, vc, inf }
    }
    pub fn new_directed(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost));
        }
        Self { graph, vc, inf }
    }
    pub fn new_empty(vc: usize, inf: Cost) -> Self {
        let graph = vec![Vec::new(); vc];
        Self { graph, vc, inf }
    }
    pub fn add_undirected_edge(&mut self, e: impl Into<Edge<Cost>>) {
        let e = e.into();
        self.graph[e.from].push((e.to, e.cost));
        self.graph[e.to].push((e.from, e.cost));
    }
    pub fn add_directed_edge(&mut self, e: impl Into<Edge<Cost>>) {
        let e = e.into();
        self.graph[e.from].push((e.to, e.cost));
    }
    pub fn rev(&self) -> Graph<Cost> {
        let mut edges: Vec<Edge<Cost>> = Vec::new();
        for u in 0..self.vc {
            for &(v, w) in &self.graph[u] {
                edges.push((v, u, w).into());
            }
        }
        Self::new_directed(edges, self.vc, self.inf)
    }
    pub fn reachable_vertexes(&self, s: usize) -> std::collections::HashSet<usize> {
        let mut visited = std::collections::HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        visited.insert(s);
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for &(v, _) in &self.graph[u] {
                if visited.contains(&v) {
                    continue;
                }
                visited.insert(v);
                queue.push_back(v);
            }
        }
        visited
    }
    pub fn edges<'a>(&self, u: usize) -> &Vec<(usize, Cost)> {
        &self.graph[u]
    }
    pub fn vertex_count(&self) -> usize {
        self.vc
    }
    fn optionalize(&self, v: Vec<Cost>) -> Vec<Option<Cost>> {
        v.into_iter()
            .map(|x| if x == self.inf { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn prim(&self) -> Cost {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();
        let mut res = Cost::zero();
        heap.push((Cost::zero(), 0));
        while let Some((weight, u)) = heap.pop() {
            if used.contains(&u) {
                continue;
            }
            used.insert(u);
            res += weight;
            for &(v, w) in &self.graph[u] {
                if used.contains(&v) {
                    continue;
                }
                heap.push((w, v));
            }
        }
        res
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1, i64); m]
    };

    let graph = Graph::new_undirected(edges, n, 1_i64 << 60);
    let res = graph.prim();
    println!("{}", res);
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
