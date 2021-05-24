#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use whiteread::{parse_line};

const H: usize = 30;
const W: usize = 30;

type Path = Vec<usize>;
struct Solver {
    graph: Vec<HashMap<usize, i64>>,
    dir: HashMap<(usize, usize), char>,
    history: Vec<(HashSet<(usize, usize)>, i64)>,
}

impl Solver {
    fn new() -> Self {
        let n = H * W;
        let mut graph = vec![HashMap::new(); n];
        let mut dir = HashMap::new();
        let cost = 4000;
        for i in 0..H {
            for j in 0..W {
                let u = Self::vertex(i, j);
                // ↑
                if i > 0 {
                    let v = Self::vertex(i - 1, j);
                    graph[u].insert(v, cost);
                    dir.insert((u, v), 'U');
                }
                // ↓
                if i + 1 < H {
                    let v = Self::vertex(i + 1, j);
                    graph[u].insert(v, cost);
                    dir.insert((u, v), 'D');
                }
                // ←
                if j > 0 {
                    let v = Self::vertex(i, j - 1);
                    graph[u].insert(v, cost);
                    dir.insert((u, v), 'L');
                }
                // →
                if j + 1 < W {
                    let v = Self::vertex(i, j + 1);
                    graph[u].insert(v, cost);
                    dir.insert((u, v), 'R');
                }
            }
        }

        Self {
            graph,
            dir,
            history: Vec::new(),
        }
    }

    fn solve(&mut self) {
        for i in 0..1000 {
            let (si, sj, ti, tj): (usize, usize, usize, usize) = parse_line().unwrap();
            let s = Self::vertex(si, sj);
            let d = self.shortest_path(s);
            let g = Self::vertex(ti, tj);
            let path = &d[g];
            let mut u = s;
            let mut res = vec![];
            for &v in path {
                res.push(self.dir[&(u, v)]);
                u = v;
            }
            println!("{}", res.iter().join(""));
            let cost: i64 = parse_line().unwrap();
            self.update(Self::vertex(si, sj), path, cost, i);
        }
    }

    fn shortest_path(&self, start: usize) -> Vec<Path> {
        let mut cost_list = vec![1_i64 << 60; self.graph.len()];
        let mut path_list = vec![vec![]; self.graph.len()];
        let mut heap = std::collections::BinaryHeap::new();
        cost_list[start] = 0;
        path_list[start] = vec![];
        heap.push(std::cmp::Reverse((0, start, vec![])));
        while let Some(std::cmp::Reverse((cost, u, path))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for (&v, &w) in &self.graph[u] {
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    let mut new_path= path.clone();
                    new_path.push(v);
                    path_list[v] = new_path.clone();
                    cost_list[v] = new_cost;
                    heap.push(std::cmp::Reverse((new_cost, v, new_path)));
                }
            }
        }
        path_list
    }

    fn update(&mut self, s: usize, path: &Path, cost: i64, i: usize) {
        let mut u = s;
        let mut path_set = HashSet::new();
        let w = cost / path.len() as i64;
        let ratio = 0.5 as f64 * (1000 - i) as f64 / 1000 as f64;
        for &v in path {
            let new_w =  ((self.graph[u][&v] as f64 * (1 as f64 - ratio) + (w as f64 * ratio)) as i64).max(1);
            self.graph[u].insert(v,  new_w);
            path_set.insert((u, v));
            u = v
        }

        for (path2_set, cost2) in &self.history {
            if path_set.is_subset(path2_set) {
                let c = cost2 - cost;
                let len = path2_set.len() - path_set.len();
                if len == 0 {
                    continue;
                }
                let w = c / len as i64;
                for &(u, v) in path2_set.difference(&path_set) {
                    let new_w =  ((self.graph[u][&v] as f64 * (1 as f64 - ratio) + (w as f64 * ratio)) as i64).max(1);
                    self.graph[u].insert(v,  new_w);
                }
            }
            if path2_set.is_subset(&path_set) {
                let c = cost - cost2;
                let len = path_set.len() - path2_set.len();
                if len == 0 {
                    continue;
                }
                let w = c / len as i64;
                for &(u, v) in path_set.difference(&path2_set) {
                    let new_w =  ((self.graph[u][&v] as f64 * (1 as f64 - ratio) + (w as f64 * ratio)) as i64).max(1);
                    self.graph[u].insert(v,  new_w);
                }
            }
        }

        self.history.push((path_set, cost));
    }

    fn vertex(i: usize, j: usize) -> usize {
        i * W + j
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            let mut solver = Solver::new();
            solver.solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
