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
#[derive(Clone)]
pub struct WeightedGraph {
    graph: Vec<Vec<(usize, i64)>>,
    vn: usize,
}
pub type WeightedEdge = (usize, usize, i64);
impl WeightedGraph {
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
    pub fn shortest_path(&self, start: usize) -> Vec<Option<i64>> {
        let mut cost_list = vec![None; self.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &(v, w) in &self.graph[u] {
                if cost_list[v].is_some() && cost_list[v].unwrap() <= cost_list[u].unwrap() + w {
                    continue;
                }
                let new_cost = cost_list[u].unwrap() + w;
                cost_list[v] = Some(new_cost);
                if w == 0 {
                    que.push_front(v);
                } else {
                    que.push_back(v);
                }
            }
        }
        cost_list
    }
}
pub struct Grid {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
    ng_char: char,
}
pub type VertexTable = std::collections::HashMap<(usize, usize), usize>;
impl Grid {
    #[allow(dead_code)]
    const UDLR_DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
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
    pub fn to_graph(&self) -> (WeightedGraph, VertexTable) {
        let mut edges = Vec::new();
        let mut vertex_table = std::collections::HashMap::new();
        let mut v = 0;
        for i in 0..self.h {
            for j in 0..self.w {
                if self.grid[i][j] == self.ng_char {
                    continue;
                }
                let from = self.gen_vertex_if_needed((i, j), &mut vertex_table, &mut v);
                for di in -2..=2 {
                    for dj in -2..=2 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let new_i = i as isize + di;
                        let new_j = j as isize + dj;
                        if new_i < 0
                            || new_i >= self.h as isize
                            || new_j < 0
                            || new_j >= self.w as isize
                        {
                            continue;
                        }
                        let is_lrud = (new_i - i as isize).abs() + (new_j - j as isize).abs() == 1;
                        let new_i = new_i as usize;
                        let new_j = new_j as usize;
                        if self.grid[new_i][new_j] == self.ng_char {
                            continue;
                        }
                        let to =
                            self.gen_vertex_if_needed((new_i, new_j), &mut vertex_table, &mut v);
                        let cost = if is_lrud { 0 } else { 1 };
                        edges.push((from, to, cost));
                    }
                }
            }
        }
        let graph = WeightedGraph::new_directed(&edges, vertex_table.len());
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
        h: usize, _: usize,
        ch: Usize1, cw: Usize1,
        dh: Usize1, dw: Usize1,
        grid: [Chars; h]
    };

    let (graph, table) = Grid::new(&grid, '#').to_graph();
    let start = *table.get(&(ch, cw)).unwrap();
    let goal = *table.get(&(dh, dw)).unwrap();
    let res = graph.shortest_path(start)[goal].unwrap_or(-1);
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
