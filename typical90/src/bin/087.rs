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
    pub fn warshall_floyd(&self, x: Cost) -> Vec<Vec<Option<Cost>>> {
        let vc = self.vc;
        let mut cost_list = vec![vec![self.inf; vc]; vc];
        for u in 0..vc {
            for &(v, w) in &self.graph[u] {
                cost_list[u][v] = if w < Cost::zero() { x } else { w };
            }
        }
        for i in 0..vc {
            cost_list[i][i] = Cost::zero();
        }
        for k in 0..vc {
            for i in 0..vc {
                for j in 0..vc {
                    cost_list[i][j] =
                        std::cmp::min(cost_list[i][j], cost_list[i][k] + cost_list[k][j]);
                }
            }
        }
        cost_list
            .into_iter()
            .map(|v| self.optionalize(v))
            .collect::<Vec<_>>()
    }
}

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

fn solve() {
    input! {
        n: usize, p: i64, k: usize,
        costs: [[i64; n]; n]
    };

    let mut edges = Vec::new();
    for u in 0..n {
        for v in u + 1..n {
            edges.push((u, v, costs[u][v]));
        }
    }
    let graph = Graph::new_undirected(edges, n, 1_i64 << 60);
    let d = graph.warshall_floyd(p + 1);
    let mut c = 0;
    for u in 0..n {
        for v in u + 1..n {
            if d[u][v].unwrap() <= p {
                c += 1;
            }
        }
    }
    if c == k {
        println!("Infinity");
        return;
    }

    let d = graph.warshall_floyd(1);
    let mut c = 0;
    for u in 0..n {
        for v in u + 1..n {
            if d[u][v].unwrap() <= p {
                c += 1;
            }
        }
    }
    if c < k {
        println!("0");
        eprintln!("c < k");
        return;
    }

    let d = graph.warshall_floyd(p);
    let mut c = 0;
    for u in 0..n {
        for v in u + 1..n {
            if d[u][v].unwrap() <= p {
                c += 1;
            }
        }
    }
    if c > k {
        println!("0");
        eprintln!("c > k");
        return;
    }

    let a = bsearch(0_i64, p + 1, |x| {
        let d = graph.warshall_floyd(x);
        let mut c = 0;
        for u in 0..n {
            for v in u + 1..n {
                if d[u][v].unwrap() <= p {
                    c += 1;
                }
            }
        }
        c > k
    });

    let b = bsearch(p + 1, 0_i64, |x| {
        let d = graph.warshall_floyd(x);
        let mut c = 0;
        for u in 0..n {
            for v in u + 1..n {
                if d[u][v].unwrap() <= p {
                    c += 1;
                }
            }
        }
        c < k
    });

    let res = b.unwrap_or(p + 1) - a.unwrap_or(0) - 1;
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
