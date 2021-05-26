#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use whiteread::{parse_line};
use rand::prelude::*;
use std::time::{Instant, Duration};

const H: usize = 30;
const W: usize = 30;
const INITIAL_COST: i64 = 4000;
const TEST_COUNT: usize = 1000;

type Path = Vec<usize>;
struct Solver {
    _start: Instant,
    graph: Vec<HashMap<usize, i64>>,
    dir: HashMap<(usize, usize), char>,
    history: Vec<(HashSet<(usize, usize)>, i64, i64)>,
    edge_set: HashSet<(usize, usize)>,
    edges: Vec<(usize, usize)>,
    edge_to_hisidx: HashMap<(usize, usize), Vec<usize>>,
    score: i64,
    rng: ThreadRng,
}

impl Solver {
    fn new() -> Self {
        let n = H * W;
        let mut graph = vec![HashMap::new(); n];
        let mut dir = HashMap::new();
        let cost = INITIAL_COST;
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
            _start: Instant::now(),
            graph,
            dir,
            history: Vec::new(),
            edge_set: HashSet::new(),
            edges: Vec::new(),
            edge_to_hisidx: HashMap::new(),
            score: 0,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        for i in 0..TEST_COUNT {
            let (si, sj, ti, tj): (usize, usize, usize, usize) = parse_line().unwrap();
            let s = Self::vertex(si, sj);
            let d = if i < 200 {
                self.shortest_path_for_opening(s)
            } else {
                self.shortest_path(s)
            };
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
            self.update_costs(Self::vertex(si, sj), path, cost, i);
        }
    }

    fn shortest_path_for_opening(&self, start: usize) -> Vec<Path> {
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
            for (&v, _) in &self.graph[u] {
                // なるべく頂点間の移動を少なくする
                let new_cost = cost + 1;
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

    fn update_costs(&mut self, s: usize, path: &Path, cost: i64, i: usize) {
        if i + 1 == TEST_COUNT {
            return;
        }

        // 与えられたコストから暫定のコストを計算する
        let mut path_set = HashSet::new();
        let mut u = s;
        for &v in path {
            let e = (u, v);
            self.edge_to_hisidx.entry(e).or_insert(Vec::new()).push(i);
            if !self.edge_set.contains(&e) {
                self.edge_set.insert(e);
                self.edges.push(e);
            }
            path_set.insert(e);
            u = v;
        }

        // 新しいコストの重み(0.0〜1.0)
        let ratio = 0.5 as f64 * (TEST_COUNT - i) as f64 / TEST_COUNT as f64;
        // コストを、パスを構成する各頂点に分配
        let w = cost / path.len() as i64;
        let mut u = s;
        for &v in path {
            let new_w = ((self.graph[u][&v] as f64 * (1 as f64 - ratio) + (w as f64 * ratio)) as i64).max(1);
            self.graph[u].insert(v,  new_w);
            u = v
        }

        let mut gen_cost = 0;
        let mut u = s;
        for v in path {
            gen_cost += self.graph[u][v];
            u = *v;
        }
        self.history.push((path_set, cost, gen_cost));
        self.apply_last_history_score();

        if i < 450 {
            return;
        }

        // ランダムにスコアを伸ばす
        let now = Instant::now();
        let duration = if i >= 550 {
            // 3 * 450 = 1350
            Duration::from_micros(3000)
        } else {
            // 2 * 100 = 200
            Duration::from_micros(2000)
        };
        while Instant::now() - now < duration {
            for _ in 0..10 {
                let i = self.rng.gen::<usize>() % self.edges.len();
                let uv = self.edges[i];
                let (u, v) = uv;
                let cur_cost = self.graph[u][&v];
                let new_cost = (cur_cost + self.rng.gen_range(-100, 100)).max(1);
                self.graph[u].insert(v, new_cost);
                let new_score = self.update_score((u, v), cur_cost, self.score);
                if new_score > self.score {
                    self.score = new_score;
                    // gen_cost更新
                    for i in 0..self.history.len() {
                        if !self.history[i].0.contains(&(u, v)) {
                            continue;
                        }
                        let new_gen_cost = (self.history[i].2 - cur_cost + new_cost).max(1);
                        self.history[i].2 = new_gen_cost;
                    }
                } else {
                    // restore
                    self.graph[u].insert(v, cur_cost);
                }
            }
        }
    }

    fn apply_last_history_score(&mut self) {
        let (_, cost, gen_cost) = &self.history[self.history.len() - 1];
        self.score -= (cost - gen_cost).abs();
    }

    fn update_score(&self, e: (usize, usize), cur_cost: i64, cur_score: i64) -> i64 {
        let mut diff = 0_i64;
        for i in &self.edge_to_hisidx[&e] {
            let (_, cost, gen_cost) = &self.history[*i];
            // 更新前のコスト
            let prev_path_cost = gen_cost;
            let new_path_cost = gen_cost - cur_cost + self.graph[e.0][&e.1];
            diff += (cost - prev_path_cost).abs() - (cost - new_path_cost).abs();
        }
        cur_score + diff
    }

    fn vertex(i: usize, j: usize) -> usize {
        i * W + j
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
