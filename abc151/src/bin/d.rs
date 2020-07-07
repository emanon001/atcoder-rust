#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub struct Graph {
    graph: Vec<Vec<usize>>,
    vn: usize,
}

type Edge = (usize, usize);
impl Graph {
    pub fn new(edges: &[Edge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        Self { graph, vn }
    }

    pub fn new_directed(edges: &[Edge], vn: usize) -> Self {
        let mut graph = vec![Vec::new(); vn];
        for &(u, v) in edges {
            graph[u].push(v);
        }
        Self { graph, vn }
    }

    pub fn add_directed_edge(&mut self, e: Edge) {
        self.graph[e.0].push(e.1);
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.graph[e.0].push(e.1);
        self.graph[e.1].push(e.0);
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<usize>> {
        let mut cost_list = vec![None; self.vn];
        let mut que = std::collections::VecDeque::new();
        cost_list[start] = Some(0);
        que.push_back(start);
        while let Some(u) = que.pop_front() {
            for &v in &self.graph[u] {
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
}

pub struct Grid {
    grid: Vec<Vec<char>>,
    h: usize,
    w: usize,
    ng_char: char,
    dirs: Vec<(isize, isize)>,
}

type VertexTable = std::collections::HashMap<(usize, usize), usize>;
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

    pub fn to_graph(&self) -> (Graph, VertexTable) {
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
        let graph = Graph::new_directed(&edges, vertex_table.len());
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
