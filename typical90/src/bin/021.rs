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

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let vc = self.vc;
        let unknown_id = vc + 10;
        let mut id = 0;
        let mut ids = vec![0; vc];
        let mut used = vec![false; vc];
        for u in 0..vc {
            if ids[u] == 0 {
                self.scc_dfs1(u, unknown_id, &mut id, &mut ids, &mut used);
            }
        }
        let mut u_with_id = ids.into_iter().enumerate().collect::<Vec<_>>();
        u_with_id.sort_by_key(|(_, id)| -(*id as isize));
        let rev_graph = self.rev();
        let mut groups = Vec::new();
        let mut used = vec![false; vc];
        for (u, _) in u_with_id {
            if used[u] {
                continue;
            }
            let mut group = Vec::new();
            rev_graph.scc_dfs2(u, unknown_id, &mut group, &mut used);
            groups.push(group);
        }
        groups
    }
    fn scc_dfs1(
        &self,
        u: usize,
        p: usize,
        id: &mut usize,
        ids: &mut Vec<usize>,
        used: &mut Vec<bool>,
    ) {
        used[u] = true;
        for &(v, _) in &self.graph[u] {
            if v == p {
                continue;
            }
            if used[v] {
                continue;
            }
            self.scc_dfs1(v, u, id, ids, used);
        }
        *id += 1;
        ids[u] = *id;
    }
    fn scc_dfs2(&self, u: usize, p: usize, group: &mut Vec<usize>, used: &mut Vec<bool>) {
        group.push(u);
        used[u] = true;
        for &(v, _) in &self.graph[u] {
            if v == p {
                continue;
            }
            if used[v] {
                continue;
            }
            self.scc_dfs2(v, u, group, used);
        }
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };

    let edges = edges.into_iter().map(|(u, v)| (u, v, 1)).collect::<Vec<_>>();
    let graph = Graph::new_directed(edges, n, 1_i64 << 60);
    let scc = graph.scc();
    let mut res = 0_u64;
    for g in scc {
        res += g.len() as u64 * (g.len() - 1) as u64 / 2 as u64;
    }
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
