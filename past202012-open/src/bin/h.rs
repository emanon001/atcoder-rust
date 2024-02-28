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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize,
        r: Usize1, c: Usize1,
        grid: [Chars; H],
    };

    let mut edges = vec![];
    for i in 0..H {
        for j in 0..W {
            if grid[i][j] == '#' {
                continue;
            }
            let dirs = match grid[i][j] {
                '.' => vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
                '<' => vec![(0, -1)],
                '^' => vec![(-1, 0)],
                '>' => vec![(0, 1)],
                'v' => vec![(1, 0)],
                _ => unreachable!(),
            };
            for (di, dj) in dirs {
                let (ni, nj) = (i as isize + di, j as isize + dj);
                if ni < 0 || ni >= H as isize || nj < 0 || nj >= W as isize {
                    continue;
                }
                let (ni, nj) = (ni as usize, nj as usize);
                if grid[ni][nj] == '#' {
                    continue;
                }
                edges.push((i * W + j, ni * W + nj));
            }
        }
    }
    let graph = Graph::new_directed(edges, H * W, 1 << 30).rev();
    let dist = graph.shortest_path_1(r * W + c);
    for i in 0..H {
        let mut row = vec![];
        for j in 0..W {
            row.push(if grid[i][j] == '#' {
                '#'
            } else if dist[i * W + j].is_some() {
                'o'
            } else {
                'x'
            });
        }
        println!("{}", row.iter().join(""));
    }
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
