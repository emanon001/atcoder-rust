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
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    graph: Vec<Vec<(usize, Cost)>>,
    vc: usize,
    inf: Cost,
}
#[derive(Clone)]
pub struct Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    from: usize,
    to: usize,
    cost: Cost,
}
impl<Cost> From<(usize, usize, Cost)> for Edge<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
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
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
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
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
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
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn shortest_path_1(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![None; self.vc];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(Cost::zero());
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, w) in &self.graph[u] {
                if !w.is_one() {
                    panic!("weight is not 1");
                }
                if cost_list[v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + Cost::one();
                cost_list[v] = Some(new_cost);
                que.push_back(v);
            }
        }
        cost_list
    }
}

fn solve() {
    input! {
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    };

    let graph = Graph::new_undirected(edges, n, 1_i64 << 60);
    let dist = graph.shortest_path_1(0);
    let u = (1..n).max_by_key(|&u| {
        dist[u].unwrap_or(0)
    }).unwrap();
    let res =
        graph.shortest_path_1(u)
            .into_iter()
            .filter_map(std::convert::identity)
            .max().unwrap() + 1;
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
