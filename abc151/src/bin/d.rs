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
pub struct UnweightedGraph {
    graph: WeightedGraph,
}

pub type UnweightedEdge = (usize, usize);
impl UnweightedGraph {
    pub fn new(edges: &[UnweightedEdge], vn: usize) -> Self {
        let edges = Self::to_weighted_edges(&edges);
        let graph = WeightedGraph::new(&edges, vn);
        Self { graph }
    }

    pub fn new_directed(edges: &[UnweightedEdge], vn: usize) -> Self {
        let edges = Self::to_weighted_edges(&edges);
        let graph = WeightedGraph::new_directed(&edges, vn);
        Self { graph }
    }

    pub fn add_directed_edge(&mut self, e: UnweightedEdge) {
        self.graph.add_directed_edge((e.0, e.1, 1));
    }

    pub fn add_edge(&mut self, e: UnweightedEdge) {
        self.graph.add_edge((e.0, e.1, 1));
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<usize>> {
        let mut cost_list = vec![None; self.graph.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, _) in &self.graph.graph[u] {
                if cost_list[v].is_some() {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + 1;
                cost_list[v] = Some(new_cost);
                que.push_back(v);
            }
        }
        cost_list
    }

    pub fn to_weighted_graph(&self) -> WeightedGraph {
        self.graph.clone()
    }

    fn to_weighted_edges(edges: &[UnweightedEdge]) -> Vec<WeightedEdge> {
        edges
            .into_iter()
            .map(|(u, v)| (*u, *v, 1))
            .collect::<Vec<_>>()
    }
}

#[derive(Clone)]
pub struct WeightedGraph {
    graph: Vec<Vec<(usize, i64)>>,
    vn: usize,
}

pub type WeightedEdge = (usize, usize, i64);
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

pub struct Grid {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
    ng_char: char,
    dirs: Vec<(isize, isize)>,
}

pub type VertexTable = std::collections::HashMap<(usize, usize), usize>;
impl Grid {
    // 上下左右 (i, j)
    #[allow(dead_code)]
    const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // 上下左右 + 斜め (i, j)
    #[allow(dead_code)]
    const ALL_DIRS: [(isize, isize); 8] = [
        // 時計回り
        (-1, 0),  // 上
        (-1, 1),  // 右上
        (0, 1),   // 右
        (1, 1),   // 右下
        (1, 0),   // 下
        (1, -1),  // 左下
        (0, -1),  // 左
        (-1, -1), // 左上
    ];

    pub fn new(grid: &[Vec<char>], ng_char: char, dirs: &[(isize, isize)]) -> Self {
        assert!(grid.len() > 0);
        let grid = grid.into_iter().cloned().collect::<Vec<_>>();
        let h = grid.len();
        let w = grid[0].len();
        let dirs = dirs.into_iter().copied().collect::<Vec<_>>();
        Self {
            grid,
            h,
            w,
            ng_char,
            dirs,
        }
    }

    pub fn to_graph(&self) -> (UnweightedGraph, VertexTable) {
        let mut edges = Vec::new();
        let mut vertex_table = std::collections::HashMap::new();
        let mut v = 0;
        for i in 0..self.h {
            for j in 0..self.w {
                if self.grid[i][j] == self.ng_char {
                    continue;
                }
                let from = self.gen_vertex_if_needed((i, j), &mut vertex_table, &mut v);
                for &(di, dj) in &self.dirs {
                    let new_i = i as isize + di;
                    let new_j = j as isize + dj;
                    if new_i < 0
                        || new_i >= self.h as isize
                        || new_j < 0
                        || new_j >= self.w as isize
                    {
                        continue;
                    }
                    let new_i = new_i as usize;
                    let new_j = new_j as usize;
                    if self.grid[new_i][new_j] == self.ng_char {
                        continue;
                    }
                    let to = self.gen_vertex_if_needed((new_i, new_j), &mut vertex_table, &mut v);
                    edges.push((from, to));
                }
            }
        }
        let graph = UnweightedGraph::new_directed(&edges, vertex_table.len());
        (graph, vertex_table)
    }

    fn gen_vertex_if_needed(
        &self,
        pos: (usize, usize),
        vertex_table: &mut VertexTable,
        cur_v: &mut usize,
    ) -> usize {
        if let Some(&v) = vertex_table.get(&pos) {
            return v;
        } else {
            let v = *cur_v;
            vertex_table.insert(pos, *cur_v);
            *cur_v += 1;
            return v;
        };
    }
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let (graph, vertex_table) = Grid::new(&grid, '#', &Grid::UDLR_DIRS).to_graph();
    let mut res = 0;
    for i in 0..h {
        for j in 0..w {
            if let Some(&v) = vertex_table.get(&(i, j)) {
                let d = graph.shortest_path(v);
                let max = d
                    .into_iter()
                    .filter_map(std::convert::identity)
                    .max()
                    .unwrap_or(0);
                res = res.max(max);
            }
        }
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
