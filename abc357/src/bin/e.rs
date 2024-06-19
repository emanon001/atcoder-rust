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
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Copy + num::traits::NumAssign,
{
    pub fn scc(&self) -> Vec<Vec<usize>> {
        let vc = self.vc;
        let mut id = 0;
        let mut ids = vec![0; vc];
        let mut used = vec![false; vc];
        for u in 0..vc {
            if used[u] {
                continue;
            }
            self.scc_dfs1(u, &mut id, &mut ids, &mut used);
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
            rev_graph.scc_dfs2(u, &mut group, &mut used);
            groups.push(group);
        }
        groups
    }
    fn scc_dfs1(&self, u: usize, id: &mut usize, ids: &mut Vec<usize>, used: &mut Vec<bool>) {
        used[u] = true;
        for &(v, _) in &self.graph[u] {
            if used[v] {
                continue;
            }
            self.scc_dfs1(v, id, ids, used);
        }
        *id += 1;
        ids[u] = *id;
    }
    fn scc_dfs2(&self, u: usize, group: &mut Vec<usize>, used: &mut Vec<bool>) {
        group.push(u);
        used[u] = true;
        for &(v, _) in &self.graph[u] {
            if used[v] {
                continue;
            }
            self.scc_dfs2(v, group, used);
        }
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [Usize1; N],
    };

    let mut edges: Vec<(usize, usize)> = vec![];
    for (u, v) in A.clone().into_iter().enumerate() {
        edges.push((u, v).into());
    }

    let graph = Graph::new_directed(edges, N, 1 << 30);
    let scc = graph.scc();
    let mut ans = 0;
    let mut u_to_count = HashMap::new();
    for group in &scc {
        let count = group.len();
        for u in group {
            u_to_count.insert(*u, count);
        }
    }

    let mut dp = HashMap::new();
    for group in &scc {
        let len = group.len();
        if len == 1 {
            let u = group[0];
            ans += dfs(u, &A, &u_to_count, &mut dp);
        } else {
            ans += len * len;
        }
    }
    println!("{}", ans);
}

fn dfs(
    u: usize,
    graph: &Vec<usize>,
    u_to_count: &HashMap<usize, usize>,
    dp: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(v) = dp.get(&u) {
        return *v;
    }
    let v = graph[u];
    if u == v {
        return 1;
    }
    if u_to_count[&v] > 1 {
        return 1 + u_to_count[&v];
    }
    let res = 1 + dfs(v, graph, u_to_count, dp);
    dp.insert(u, res);
    res
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
