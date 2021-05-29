#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::cmp::Reverse;
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
    cur_score: i64,
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
            cur_score: 0,
            rng: rand::thread_rng(),
        }
    }

    fn solve(&mut self) {
        for i in 0..TEST_COUNT {
            let (si, sj, ti, tj): (usize, usize, usize, usize) = parse_line().unwrap();
            let s = Self::vertex(si, sj);
            let d = if i < 150 {
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
        // なるべく頂点間の移動を少なくする
        let mut cost_list = vec![1_i64 << 60; self.graph.len()];
        let mut path_list = vec![vec![]; self.graph.len()];
        let mut heap = BinaryHeap::new();
        cost_list[start] = 0;
        path_list[start] = vec![];
        heap.push(Reverse((0, start, vec![])));
        while let Some(Reverse((cost, u, path))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for (&v, _) in &self.graph[u] {
                let new_cost = cost + 1;
                if new_cost < cost_list[v] {
                    let mut new_path= path.clone();
                    new_path.push(v);
                    path_list[v] = new_path.clone();
                    cost_list[v] = new_cost;
                    heap.push(Reverse((new_cost, v, new_path)));
                }
            }
        }
        path_list
    }

    fn shortest_path(&self, start: usize) -> Vec<Path> {
        let mut cost_list = vec![1_i64 << 60; self.graph.len()];
        let mut path_list = vec![vec![]; self.graph.len()];
        let mut heap = BinaryHeap::new();
        cost_list[start] = 0;
        path_list[start] = vec![];
        heap.push(Reverse((0, start, vec![])));
        while let Some(Reverse((cost, u, path))) = heap.pop() {
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
                    heap.push(Reverse((new_cost, v, new_path)));
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
            let re = (v, u);
            self.edge_to_hisidx.entry(e).or_insert(Vec::new()).push(i);
            self.edge_to_hisidx.entry(re).or_insert(Vec::new()).push(i);
            if self.edge_set.insert(e) {
                self.edges.push(e);
                self.edges.push(re);
            }
            path_set.insert(e);
            u = v;
        }

        // 新しいコストの重み(0.0〜0.5)
        let new_cost_ratio = 0.5 as f64 * (TEST_COUNT - i) as f64 / TEST_COUNT as f64;
        // コストを、パスを構成する辺に分配
        let w = cost / path.len() as i64;
        let mut u = s;
        for &v in path {
            let new_w = ((self.graph[u][&v] as f64 * (1 as f64 - new_cost_ratio) + (w as f64 * new_cost_ratio)) as i64).max(1);
            self.graph[u].insert(v,  new_w);
            self.graph[v].insert(u,  new_w);
            u = v
        }

        let mut calc_cost = 0;
        let mut u = s;
        for v in path {
            calc_cost += self.graph[u][v];
            u = *v;
        }
        self.history.push((path_set, cost, calc_cost));
        self.apply_last_history_score();

        let random_start = 150;
        if i < random_start {
            return;
        }

        // ランダムにスコアを伸ばす
        // 1600msを目安にする
        let now = Instant::now();
        let duration = Duration::from_micros(1630000_u64 / (TEST_COUNT - random_start) as u64);
        while Instant::now() - now < duration {
            for _ in 0..10 {
                let i = self.rng.gen::<usize>() % self.edges.len();
                let uv = self.edges[i];
                let (u, v) = uv;
                let cur_cost = self.graph[u][&v];
                let new_cost = (cur_cost + self.rng.gen_range(-100, 100)).max(1);
                self.graph[u].insert(v, new_cost);
                let new_score = self.update_score((u, v), cur_cost, self.cur_score);
                if new_score > self.cur_score {
                    self.cur_score = new_score;
                    // cost更新
                    for i in 0..self.history.len() {
                        if !self.history[i].0.contains(&(u, v)) {
                            continue;
                        }
                        let new_calc_cost = (self.history[i].2 - cur_cost + new_cost).max(1);
                        self.history[i].2 = new_calc_cost;
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
        self.cur_score -= (cost - gen_cost).abs();
    }

    fn update_score(&self, e: (usize, usize), cur_cost: i64, cur_score: i64) -> i64 {
        let mut diff = 0_i64;
        for i in &self.edge_to_hisidx[&e] {
            let (_, cost, calc_cost) = &self.history[*i];
            // 更新前のコスト
            let prev_path_cost = calc_cost;
            let new_path_cost = calc_cost - cur_cost + self.graph[e.0][&e.1];
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
