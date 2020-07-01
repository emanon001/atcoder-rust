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
    vn: usize,
}

type WeightedEdge = (usize, usize, i64);
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

    pub fn new_directed(edges: &[WeightedEdge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v, w) in edges {
            graph[u].push((v, w));
        }
        Self { graph, vn }
    }

    pub fn add_directed_edge(&mut self, e: WeightedEdge) {
        self.graph[e.0].push((e.1, e.2));
    }

    pub fn add_edge(&mut self, e: WeightedEdge) {
        self.graph[e.0].push((e.1, e.2));
        self.graph[e.1].push((e.0, e.2));
    }

    pub fn bellman_ford(&self, s: usize) -> Option<Vec<Option<i64>>> {
        let vn = self.vn;
        let inf = Self::INF;
        let mut cost_list = vec![inf; vn];
        cost_list[s] = 0;
        for c in 0..vn {
            for u in 0..vn {
                for &(v, w) in &self.graph[u] {
                    if cost_list[u] == inf {
                        continue;
                    }
                    let new_cost = cost_list[u] + w;
                    if new_cost < cost_list[v] {
                        cost_list[v] = new_cost;
                        if c == vn - 1 {
                            return None;
                        }
                    }
                }
            }
        }
        Some(self.optionalize(cost_list))
    }

    pub fn prim(&self) -> i64 {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();

        let mut res = 0_i64;
        heap.push(std::cmp::Reverse((0_i64, 0)));
        while let Some(std::cmp::Reverse((weight, u))) = heap.pop() {
            if used.contains(&u) {
                continue;
            }
            used.insert(u);
            res += weight;
            for &(v, w) in &self.graph[u] {
                if used.contains(&v) {
                    continue;
                }
                heap.push(std::cmp::Reverse((w, v)));
            }
        }
        res
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

    pub fn rev(&self) -> WeightedGraph {
        let mut edges = Vec::new();
        for u in 0..self.vn {
            for &(v, w) in &self.graph[u] {
                edges.push((v, u, w));
            }
        }
        Self::new_directed(&edges, self.vn)
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![Self::INF; self.vn];
        let mut heap = std::collections::BinaryHeap::new();

        cost_list[start] = 0;
        heap.push(std::cmp::Reverse((0_i64, start)));

        while let Some(std::cmp::Reverse((cost, u))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    heap.push(std::cmp::Reverse((new_cost, v)));
                    cost_list[v] = new_cost;
                }
            }
        }
        self.optionalize(cost_list)
    }

    pub fn warshall_floyd(&self) -> Vec<Vec<Option<i64>>> {
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
            .into_iter()
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
        n: usize, m: usize, p: i64,
        edges: [(Usize1, Usize1, i64); m]
    };

    let graph = WeightedGraph::new_directed(&edges, n);
    let rev_graph = graph.rev();
    let s_v = graph.reachable_vertexes(0);
    let e_v = rev_graph.reachable_vertexes(n - 1);
    let vset = s_v.intersection(&e_v).collect::<HashSet<_>>();
    let filtered_edges = edges
        .into_iter()
        .filter(|(a, b, _)| vset.contains(a) && vset.contains(b))
        .map(|(a, b, c)| (a, b, -(c - p)))
        .collect::<Vec<_>>();
    let filtered_graph = WeightedGraph::new_directed(&filtered_edges, n);
    let res = if let Some(d) = filtered_graph.bellman_ford(0) {
        std::cmp::max(-d[n - 1].unwrap(), 0)
    } else {
        -1
    };
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
