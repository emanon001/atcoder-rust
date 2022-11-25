#[allow(unused_imports)]
use itertools::Itertools;
use num::traits::Pow;
#[allow(unused_imports)]
use num::*;
use ordered_float::NotNaN;
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
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    pub fn traveling_salesman(&self, start: usize) -> Cost {
        let mut dp = vec![vec![None; self.vc]; 1 << self.vc];
        let fin = (1 << self.vc) - 1;
        self.traveling_salesman_impl(0, start, &mut dp, start, fin)
    }
    fn traveling_salesman_impl(
        &self,
        state: usize,
        u: usize,
        dp: &mut [Vec<Option<Cost>>],
        start: usize,
        fin: usize,
    ) -> Cost {
        if let Some(res) = dp[state][u] {
            return res;
        }
        if state == fin && u == start {
            let res = Cost::zero();
            dp[state][u] = Some(res);
            return res;
        }
        let mut res = self.inf;
        for &(v, cost) in &self.graph[u] {
            let new_state = state | (1 << v);
            if new_state != state {
                let cost = self.traveling_salesman_impl(new_state, v, dp, start, fin) + cost;
                res = if res < cost { res } else { cost };
            }
        }
        dp[state][u] = Some(res);
        res
    }
}

fn solve() {
    input! {
        n: usize,
        points: [(f64, f64); n]
    };

    let mut graph = Graph::new_empty(n, (1 << 30) as f64);
    for i in 0..n {
        let (x1, y1) = points[i];
        for j in i..n {
            let (x2, y2) = points[j];
            let distance = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
            graph.add_undirected_edge((i, j, distance));
        }
    }
    let res = graph.traveling_salesman(0);
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
