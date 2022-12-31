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
}

pub struct Grid<T>
where
    T: PartialEq + Eq + Copy,
{
    grid: Vec<Vec<T>>,
    h: usize,
    w: usize,
    ng: Option<T>,
}
impl<T> Grid<T>
where
    T: PartialEq + Eq + Copy,
{
    pub fn new(grid: Vec<Vec<T>>, ng: impl Into<Option<T>>) -> Self {
        assert!(grid.len() > 0);
        let h = grid.len();
        let w = grid[0].len();
        let ng: Option<T> = ng.into();
        Self { grid, h, w, ng }
    }
    pub fn to_graph<Cost, F>(&self, inf: Cost, generator: F) -> Graph<Cost>
    where
        Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
        F: Fn(&Grid<T>, usize, usize) -> Vec<GridDestination<Cost>>,
    {
        let mut edges: Vec<Edge<Cost>> = Vec::new();
        for i in 0..self.h {
            for j in 0..self.w {
                let from = self.vertex(i, j);
                for (pos, w) in generator(&self, i, j) {
                    let to = self.vertex(pos.0, pos.1);
                    edges.push((from, to, w).into());
                }
            }
        }
        Graph::new_directed(edges, self.h * self.w, inf)
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
    pub fn cell(&self, i: usize, j: usize) -> T {
        self.grid[i][j]
    }
    pub fn ng(&self) -> Option<T> {
        self.ng
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
pub type GridDestination<Cost> = (GridPos, Cost);
pub fn gen_grid_destinations<T, Cost>(
    grid: &Grid<T>,
    i: usize,
    j: usize,
    directions: &[(isize, isize)],
) -> Vec<GridDestination<Cost>>
where
    T: PartialEq + Eq + Copy,
    Cost: PartialOrd + Ord + Copy + num::traits::NumAssign,
{
    let mut dest = Vec::new();
    if grid.ng().is_some() && grid.cell(i, j) == grid.ng().unwrap() {
        return dest;
    }
    for &(di, dj) in directions {
        let new_i = i as isize + di;
        let new_j = j as isize + dj;
        if !grid.in_grid(new_i, new_j) {
            continue;
        }
        let new_i = new_i as usize;
        let new_j = new_j as usize;
        if grid.ng().is_some() && grid.cell(new_i, new_j) == grid.ng().unwrap() {
            continue;
        }
        dest.push(((new_i, new_j), Cost::one()));
    }
    dest
}

fn solve() {
    input! {
        r: usize, c: usize,
        sy: Usize1, sx: Usize1,
        gy: Usize1, gx: Usize1,
        grid: [Chars; r]
    };

    let grid = Grid::new(grid, '#');
    let s = grid.vertex(sy, sx);
    let g = grid.vertex(gy, gx);
    let graph = grid.to_graph(1_usize << 30, |g, i, j| {
        gen_grid_destinations(g, i, j, &UDLR_DIRS)
    });
    let res = graph.shortest_path_1(s)[g].unwrap();
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
