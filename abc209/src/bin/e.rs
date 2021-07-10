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
    pub fn new(edges: Vec<impl Into<Edge<Cost>>>, vc: usize, inf: Cost) -> Self {
        let mut graph = vec![Vec::new(); vc];
        for e in edges {
            let e = e.into();
            graph[e.from].push((e.to, e.cost));
            graph[e.to].push((e.from, e.cost));
        }
        Self { graph, vc, inf }
    }
    pub fn new_noedge(vc: usize, inf: Cost) -> Self {
        let graph = vec![Vec::new(); vc];
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
    pub fn add_directed_edge(&mut self, e: impl Into<Edge<Cost>>) {
        let e = e.into();
        self.graph[e.from].push((e.to, e.cost));
    }
    pub fn add_edge(&mut self, e: impl Into<Edge<Cost>>) {
        let e = e.into();
        self.graph[e.from].push((e.to, e.cost));
        self.graph[e.to].push((e.from, e.cost));
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
    pub fn vertex_count(&self) -> usize {
        self.vc
    }
}

impl<Cost> Graph<Cost>
where
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
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

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        sv: [String; n]
    };
    let mut vertex_map = HashMap::new();
    let mut vertex = 0_usize;
    for s in &sv {
        if !vertex_map.contains_key(s) {
            vertex_map.insert(s.to_string(), vertex);
            vertex += 1;
        }
        let head = s[0..3].to_string();
        if !vertex_map.contains_key(&head) {
            vertex_map.insert(head, vertex);
            vertex += 1;
        }
    }
    let mut edges = Vec::new();
    let mut used = HashSet::new();
    for s in &sv {
        let su = vertex_map[s];
        let head = s[0..3].to_string();
        let tail = s[s.len() - 3..].to_string();
        let u = vertex_map[&head];
        if vertex_map.get(&tail).is_none() {
            continue;
        }
        let v = vertex_map[&tail];
        if used.contains(&(u, v)) {
            continue;
        }
        used.insert((u, v));
        edges.push((su, v));
        edges.push((u, v));
    }
    let mut dfs_graph = vec![Vec::new(); vertex];
    for &(u, v) in &edges {
        dfs_graph[u].push(v);
    }
    let graph = Graph::new_directed(edges, vertex, 1_i64 << 60);
    let scc = graph.scc();
    let mut cycle_set = HashSet::new();
    for g in scc {
        if g.len() < 2 {
            continue;
        }
        for u in g {
            cycle_set.insert(u);
        }
    }
    let mut dp = vec![vec![None; 2]; vertex];
    for s in sv {
        let u = vertex_map[&s];
        let res = dfs(u, 0, &dfs_graph, &cycle_set, &mut dp);
        let res = match res {
            -1 => "Aoki",
            0 => "Draw",
            1 => "Takahashi",
            _ => unreachable!()
        };
        println!("{}", res);
    }
}

fn dfs(u: usize, turn: usize, graph: &Vec<Vec<usize>>, cycle_set: &HashSet<usize>, dp: &mut Vec<Vec<Option<isize>>>) -> isize {
    if let Some(res) = dp[u][turn] {
        return res;
    }

    // -1: 負け, 0: 引き分け, 1: 勝ち
    if graph[u].len() == 0 {
        let res = 1;
        dp[u][turn] = Some(res);
        return res;
    }
    let mut res = if cycle_set.contains(&u) { 0 } else { -1 };
    for &v in &graph[u] {
        if u == v {
            chmax!(res, 0);
            continue;
        }
        let res2 = dfs(v, (turn + 1) % 2, graph, cycle_set, dp);
        chmax!(res, -res2);
    }
    dp[u][turn] = Some(res);
    return res;
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
