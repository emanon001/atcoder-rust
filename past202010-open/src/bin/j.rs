#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
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
    pub fn shortest_path(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![self.inf; self.vc];
        let mut heap = std::collections::BinaryHeap::new();
        cost_list[start] = Cost::zero();
        heap.push((std::cmp::Reverse(Cost::zero()), start));
        while let Some((std::cmp::Reverse(cost), u)) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    heap.push((std::cmp::Reverse(new_cost), v));
                    cost_list[v] = new_cost;
                }
            }
        }
        self.optionalize(cost_list)
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        XAB: i64, XAC: i64, XBC: i64,
        S: Chars,
        ABC: [(Usize1, Usize1, i64); M],
    };

    let mut warp_a = vec![];
    let mut warp_b = vec![];
    let mut warp_c = vec![];
    for (i, ch) in S.into_iter().enumerate() {
        match ch {
            'A' => warp_a.push(i),
            'B' => warp_b.push(i),
            'C' => warp_c.push(i),
            _ => unreachable!(),
        };
    }
    let mut graph = Graph::new_undirected(ABC, N + 3 * 2, 1_i64 << 60);
    for u in warp_a {
        // A → AB
        // BA → A
        graph.add_directed_edge((u, N, 0));
        graph.add_directed_edge((N + 1, u, XAB));
        // A → AC
        // CA → A
        graph.add_directed_edge((u, N + 2, 0));
        graph.add_directed_edge((N + 3, u, XAC));
    }
    for u in warp_b {
        // B → BA
        // AB → B
        graph.add_directed_edge((u, N + 1, 0));
        graph.add_directed_edge((N, u, XAB));
        // B → BC
        // CB → B
        graph.add_directed_edge((u, N + 4, 0));
        graph.add_directed_edge((N + 5, u, XBC));
    }
    for u in warp_c {
        // C → CA
        // AC → C
        graph.add_directed_edge((u, N + 3, 0));
        graph.add_directed_edge((N + 2, u, XAC));
        // C → CB
        // BC → C
        graph.add_directed_edge((u, N + 5, 0));
        graph.add_directed_edge((N + 4, u, XBC));
    }
    let dist = graph.shortest_path(0);
    // eprintln!("{:?}", dist);
    let ans = dist[N - 1].unwrap();
    println!("{}", ans);
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
