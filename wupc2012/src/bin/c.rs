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
        n: usize, m: usize,
        grid: [Chars; n]
    };

    let mut s = (0, 0);
    let mut c = (0, 0);
    let mut g = (0, 0);
    for i in 0..n {
        for j in 0..m {
            match grid[i][j] {
                'S' => s = (i, j),
                'C' => c = (i, j),
                'G' => g = (i, j),
                _ => {}
            }
        }
    }
    let grid = Grid::new(grid, '#');
    let s = grid.vertex(s.0, s.1);
    let c = grid.vertex(c.0, c.1);
    let g = grid.vertex(g.0, g.1);
    let graph = grid.to_graph(1_usize << 30, |grid, i, j| {
        gen_grid_destinations(grid, i, j, &UDLR_DIRS)
    });
    let res = match (graph.shortest_path_1(s)[c], graph.shortest_path_1(c)[g]){
        (Some(a), Some(b)) => (a + b) as isize,
        _ => -1
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
