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
pub struct Graph {
    graph: WeightedGraph,
}
pub type Edge = (usize, usize);
impl Graph {
    pub fn new(edges: &[Edge], vn: usize) -> Self {
        let edges = Self::to_weighted_edges(&edges);
        let graph = WeightedGraph::new(&edges, vn);
        Graph { graph }
    }
    pub fn new_directed(edges: &[Edge], vn: usize) -> Self {
        let edges = Self::to_weighted_edges(&edges);
        let graph = WeightedGraph::new_directed(&edges, vn);
        Graph { graph }
    }
    pub fn add_directed_edge(&mut self, e: Edge) {
        self.graph.add_directed_edge((e.0, e.1, 1));
    }
    pub fn add_edge(&mut self, e: Edge) {
        self.graph.add_edge((e.0, e.1, 1));
    }
    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        self.graph.shortest_path_1(start)
    }
    pub fn to_weighted_graph(&self) -> WeightedGraph {
        self.graph.clone()
    }
    fn to_weighted_edges(edges: &[Edge]) -> Vec<WeightedEdge> {
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
    pub fn shortest_path_1(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![None; self.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, w) in &self.graph[u] {
                if w != 1 {
                    panic!("weight is not 1");
                }
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
    pub fn shortest_path_01(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![Self::INF; self.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = 0;
        que.push_front((start, 0_i64));
        while let Some((u, cost)) = que.pop_front() {
            if cost > cost_list[u] {
                continue;
            }
            for &(v, w) in &self.graph[u] {
                if w != 0 && w != 1 {
                    panic!("weight is not 01");
                }
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    cost_list[v] = new_cost;
                    if w == 0 {
                        que.push_front((v, new_cost));
                    } else {
                        que.push_back((v, new_cost));
                    }
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
}
impl Grid {
    pub fn new(grid: &[Vec<char>], ng_char: char) -> Self {
        assert!(grid.len() > 0);
        let grid = grid.into_iter().cloned().collect::<Vec<_>>();
        let h = grid.len();
        let w = grid[0].len();
        Self {
            grid,
            h,
            w,
            ng_char,
        }
    }
    pub fn to_graph<F>(&self, generator: F) -> WeightedGraph
    where
        F: Fn(&Grid, usize, usize) -> Vec<GridDestination>,
    {
        let mut edges = Vec::new();
        for i in 0..self.h {
            for j in 0..self.w {
                let from = self.vertex(i, j);
                for (pos, w) in generator(&self, i, j) {
                    let to = self.vertex(pos.0, pos.1);
                    edges.push((from, to, w));
                }
            }
        }
        WeightedGraph::new_directed(&edges, self.h * self.w)
    }
    pub fn height(&self) -> usize {
        self.h
    }
    pub fn width(&self) -> usize {
        self.w
    }
    pub fn in_grid(&self, i: isize, j: isize) -> bool {
        i >= 0 && i < self.h as isize && j >= 0 && j < self.w as isize
    }
    pub fn cell(&self, i: usize, j: usize) -> char {
        self.grid[i][j]
    }
    pub fn ng_char(&self) -> char {
        self.ng_char
    }
    pub fn vertex(&self, i: usize, j: usize) -> usize {
        i * self.w + j
    }
}
/// 上下左右 (i, j)
pub const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
/// 上下左右 + 斜め (i, j)
pub const ALL_DIRS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];
pub type GridPos = (usize, usize);
pub type GridDestination = (GridPos, i64);
pub fn gen_grid_destinations(
    grid: &Grid,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<GridDestination> {
    let mut dest = Vec::new();
    for &(di, dj) in directions {
        let new_i = i as isize + di;
        let new_j = j as isize + dj;
        if !grid.in_grid(new_i, new_j) {
            continue;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if grid.cell(new_i, new_j) == grid.ng_char() {
            continue;
        }
        dest.push(((new_i, new_j), 1));
    }
    dest
}

fn solve() {
    input! {
        h: usize, w: usize,
        grid: [Chars; h]
    };

    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match grid[i][j] {
                's' => s = (i, j),
                'g' => g = (i, j),
                _ => {}
            };
        }
    }

    let grid = Grid::new(&grid, '#');
    let graph = grid.to_graph(|grid, i, j| {
        let mut dest = Vec::new();
        for &(di, dj) in &UDLR_DIRS {
            let new_i = i as isize + di;
            let new_j = j as isize + dj;
            if !grid.in_grid(new_i, new_j) {
                continue;
            }
            let new_i = new_i as usize;
            let new_j = new_j as usize;
            let cost = if grid.cell(new_i, new_j) == grid.ng_char() {
                1
            } else {
                0
            };
            dest.push(((new_i, new_j), cost));
        }
        dest
    });
    let s = grid.vertex(s.0, s.1);
    let g = grid.vertex(g.0, g.1);
    let cost = graph.shortest_path_01(s)[g].unwrap();
    let res = if cost <= 2 { "YES" } else { "NO" };
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
