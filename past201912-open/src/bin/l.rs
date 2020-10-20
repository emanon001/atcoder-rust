#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use ordered_float::NotNan;
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
pub type Edge<Cost> = (usize, usize, Cost);
impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn new(edges: Vec<Edge<Cost>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for (u, v, w) in edges {
            graph[u].push((v, w));
            graph[v].push((u, w));
        }
        Self { graph, vc, inf }
    }
    pub fn new_directed(edges: Vec<Edge<Cost>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for (u, v, w) in edges {
            graph[u].push((v, w));
        }
        Self { graph, vc, inf }
    }
    pub fn add_directed_edge(&mut self, e: Edge<Cost>) {
        self.graph[e.0].push((e.1, e.2));
    }
    pub fn add_edge(&mut self, e: Edge<Cost>) {
        self.graph[e.0].push((e.1, e.2));
        self.graph[e.1].push((e.0, e.2));
    }
    pub fn bellman_ford(&self, s: usize) -> Option<Vec<Option<Cost>>> {
        let vc = self.vc;
        let mut cost_list = vec![self.inf; vc];
        cost_list[s] = Cost::zero();
        for c in 0..vc {
            for u in 0..vc {
                for &(v, w) in &self.graph[u] {
                    if cost_list[u] == self.inf {
                        continue;
                    }
                    let new_cost = cost_list[u] + w;
                    if new_cost < cost_list[v] {
                        cost_list[v] = new_cost;
                        if c == vc - 1 {
                            return None;
                        }
                    }
                }
            }
        }
        Some(self.optionalize(cost_list))
    }
    pub fn prim(&self) -> Cost {
        let mut used = std::collections::HashSet::new();
        let mut heap = std::collections::BinaryHeap::new();
        let mut res = Cost::zero();
        heap.push(std::cmp::Reverse((Cost::zero(), 0)));
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
    pub fn rev(&self) -> Graph<Cost> {
        let mut edges = Vec::new();
        for u in 0..self.vc {
            for &(v, w) in &self.graph[u] {
                edges.push((v, u, w));
            }
        }
        Self::new_directed(edges, self.vc, self.inf)
    }
    pub fn shortest_path(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![self.inf; self.vc];
        let mut heap = std::collections::BinaryHeap::new();
        cost_list[start] = Cost::zero();
        heap.push(std::cmp::Reverse((Cost::zero(), start)));
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
    pub fn shortest_path_01(&self, start: usize) -> Vec<Option<Cost>> {
        let mut cost_list = vec![self.inf; self.vc];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Cost::zero();
        que.push_front((start, Cost::zero()));
        while let Some((u, cost)) = que.pop_front() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                if !w.is_zero() && !w.is_one() {
                    panic!("weight is not 01");
                }
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    cost_list[v] = new_cost;
                    if w.is_zero() {
                        que.push_front((v, new_cost));
                    } else {
                        que.push_back((v, new_cost));
                    }
                }
            }
        }
        self.optionalize(cost_list)
    }
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
                res = res.min(cost);
            }
        }
        dp[state][u] = Some(res);
        res
    }
    pub fn vertex_count(&self) -> usize {
        self.vc
    }
    pub fn warshall_floyd(&self) -> Vec<Vec<Option<Cost>>> {
        let vc = self.vc;
        let mut cost_list = vec![vec![self.inf; vc]; vc];
        for u in 0..vc {
            for &(v, w) in &self.graph[u] {
                cost_list[u][v] = w;
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
    fn optionalize(&self, v: Vec<Cost>) -> Vec<Option<Cost>> {
        v.into_iter()
            .map(|x| if x == self.inf { None } else { Some(x) })
            .collect::<Vec<_>>()
    }
}

fn weight(a: (f64, f64, usize), b: (f64, f64, usize)) -> NotNan<f64> {
    let xd = (a.0 - b.0).abs();
    let yd = (a.1 - b.1).abs();
    let d = (xd * xd + yd * yd).sqrt();
    let v = if a.2 == b.2 { d } else { d * 10.0 };
    NotNan::new(v).unwrap()
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
    let inf = NotNan::new((1_i64 << 50) as f64).unwrap();
    let graph = Graph::new(edges, n + m, inf);
    let mut res = NotNan::new(std::f64::MAX).unwrap();
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
