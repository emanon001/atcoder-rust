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
const N: usize = H * W;
const INITIAL_COST: i64 = 4000;
const QUERY_COUNT: usize = 1000;
const FULL_QUERY_START_NO: usize = 150;

type Path = VecDeque<usize>;
struct Solver {
    start_time: Instant,
    graph: Vec<HashMap<usize, i64>>,
    dir: HashMap<(usize, usize), char>,
    history: Vec<(HashSet<(usize, usize)>, i64, i64)>,
    edge_set: HashSet<(usize, usize)>,
    edges: Vec<(usize, usize)>,
    edge_to_hisidx: HashMap<(usize, usize), Vec<usize>>,
    cur_cost_diff: i64,
    rng: ThreadRng,
    full_query_start_time: Instant,
    max_duration_each_full_query: Duration,
}

impl Solver {
    fn new() -> Self {
        let mut graph = vec![HashMap::new(); N];
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

        let now = Instant::now();
        Self {
            start_time: now,
            graph,
            dir,
            history: Vec::new(),
            edge_set: HashSet::new(),
            edges: Vec::new(),
            edge_to_hisidx: HashMap::new(),
            cur_cost_diff: 0,
            rng: rand::thread_rng(),
            full_query_start_time: now, // dummy
            max_duration_each_full_query: Duration::from_millis(0),
        }
    }

    fn solve(&mut self) {
        for i in 0..QUERY_COUNT {
            let query_start_time = Instant::now();
            if i == FULL_QUERY_START_NO {
                // 乱択を含むクエリ処理にかけられる時間
                let duration = self.start_time + Duration::from_millis(1950) - query_start_time;
                self.max_duration_each_full_query = duration / (QUERY_COUNT - FULL_QUERY_START_NO - 1) as u32;
                self.full_query_start_time = query_start_time;
            }
            let query_duration = if i < FULL_QUERY_START_NO {
                Duration::from_millis(0)
            } else {
                let limit = self.full_query_start_time + self.max_duration_each_full_query * (i - FULL_QUERY_START_NO + 1) as u32;
                if query_start_time > limit {
                    Duration::from_millis(0)
                } else  {
                    limit - query_start_time
                }
            };
            let (si, sj, ti, tj): (usize, usize, usize, usize) = parse_line().unwrap();
            let s = Self::vertex(si, sj);
            let g = Self::vertex(ti, tj);
            let path = if i < FULL_QUERY_START_NO {
                self.shortest_path_for_opening(s, g)
            } else {
                self.shortest_path(s, g)
            };
            let mut path_dirs = vec![];
            let mut u = s;
            for &v in &path {
                path_dirs.push(self.dir[&(u, v)]);
                u = v;
            }
            println!("{}", path_dirs.iter().join(""));
            let cost: i64 = parse_line().unwrap();
            self.update_costs(Self::vertex(si, sj), path, cost, i, query_start_time, query_duration);
        }
    }

    fn shortest_path_for_opening(&self, start: usize, goal: usize) -> Path {
        // なるべく頂点間の移動を少なくする
        let mut cost_list = vec![1_i64 << 60; self.graph.len()];
        let mut from_edge_list = vec![N; self.graph.len()];
        let mut heap = BinaryHeap::new();
        cost_list[start] = 0;
        heap.push(Reverse((0, start)));
        while let Some(Reverse((cost, u))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for (&v, _) in &self.graph[u] {
                let new_cost = cost + 1;
                if new_cost < cost_list[v] {
                    from_edge_list[v] = u;
                    cost_list[v] = new_cost;
                    heap.push(Reverse((new_cost, v)));
                }
            }
        }
        let mut path: Path = VecDeque::new();
        let mut from = goal;
        while from != N {
            path.push_front(from);
            from = from_edge_list[from];
        }
        path.pop_front();
        path
    }

    fn shortest_path(&self, start: usize, goal: usize) -> Path {
        let mut cost_list = vec![1_i64 << 60; self.graph.len()];
        let mut from_edge_list = vec![N; self.graph.len()];
        let mut heap = BinaryHeap::new();
        cost_list[start] = 0;
        heap.push(Reverse((0, start)));
        while let Some(Reverse((cost, u))) = heap.pop() {
            if cost > cost_list[u] {
                continue;
            }
            for (&v, &w) in &self.graph[u] {
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    from_edge_list[v] = u;
                    cost_list[v] = new_cost;
                    heap.push(Reverse((new_cost, v)));
                }
            }
        }
        let mut path: Path = VecDeque::new();
        let mut from = goal;
        while from != N {
            path.push_front(from);
            from = from_edge_list[from];
        }
        path.pop_front();
        path
    }

    fn update_costs(&mut self, s: usize, path: Path, path_cost: i64, i: usize, query_start_time: Instant, query_duration: Duration) {
        if i + 1 == QUERY_COUNT {
            return;
        }

        // 辺とパスの対応を記録
        // 登場した辺を記録
        let mut path_set = HashSet::new();
        let mut u = s;
        for &v in &path {
            let e = (u, v);
            let re = (v, u);
            self.edge_to_hisidx.entry(e).or_insert(Vec::new()).push(i);
            self.edge_to_hisidx.entry(re).or_insert(Vec::new()).push(i);
            if self.edge_set.insert(e) {
                self.edges.push(e);
            }
            path_set.insert(e);
            u = v;
        }

        // 履歴にパスを追加
        let mut calculated_path_cost = 0;
        let mut u = s;
        for v in &path {
            calculated_path_cost += self.graph[u][v];
            u = *v;
        }
        self.history.push((path_set, path_cost, calculated_path_cost));
        self.apply_last_history_cost();

        // 与えられたコストから暫定のコストを計算する
        // 新しいコストの重み(0.0〜0.5)
        let new_cost_ratio = 0.5 as f64 * (QUERY_COUNT - i) as f64 / QUERY_COUNT as f64;
        // コストを、パスを構成する辺に分配
        let avg_edge_cost = path_cost / path.len() as i64;
        let mut u = s;
        for &v in &path {
            // cost更新
            let cur_edge_cost = self.graph[u][&v];
            let new_edge_cost = ((cur_edge_cost as f64 * (1 as f64 - new_cost_ratio) + (avg_edge_cost as f64 * new_cost_ratio)) as i64).max(1);
            self.cur_cost_diff = self.calc_cost_diff((u, v), new_edge_cost, cur_edge_cost);
            self.graph[u].insert(v,  new_edge_cost);
            self.graph[v].insert(u,  new_edge_cost);
            for &i in &self.edge_to_hisidx[&(u, v)] {
                let new_path_cost = (self.history[i].2 - cur_edge_cost + new_edge_cost).max(1);
                self.history[i].2 = new_path_cost;
            }
            u = v
        }

        // ランダムにスコアを伸ばす
        if i < FULL_QUERY_START_NO {
            return;
        }
        while Instant::now() - query_start_time < query_duration {
            for _ in 0..10 {
                let i = self.rng.gen::<usize>() % self.edges.len();
                let uv = self.edges[i];
                let (u, v) = uv;
                let cur_edge_cost = self.graph[u][&v];
                let new_edge_cost = (cur_edge_cost + self.rng.gen_range(-100, 100)).max(1);
                let new_cost_diff = self.calc_cost_diff((u, v), new_edge_cost, cur_edge_cost);
                if new_cost_diff < self.cur_cost_diff {
                    self.graph[u].insert(v, new_edge_cost);
                    self.graph[v].insert(u, new_edge_cost);
                    self.cur_cost_diff = new_cost_diff;
                    // cost更新
                    for i in 0..self.history.len() {
                        let h = &self.history[i];
                        if !h.0.contains(&(u, v)) && !h.0.contains(&(v, u)) {
                            continue;
                        }
                        let new_path_cost = (self.history[i].2 - cur_edge_cost + new_edge_cost).max(1);
                        self.history[i].2 = new_path_cost;
                    }
                }
            }
        }
    }

    fn apply_last_history_cost(&mut self) {
        let (_, path_cost, calculated_path_cost) = &self.history[self.history.len() - 1];
        self.cur_cost_diff += (path_cost - calculated_path_cost).abs();
    }

    fn calc_cost_diff(&self, e: (usize, usize), new_edge_cost: i64, cur_edge_cost: i64) -> i64 {
        let mut new_diff = self.cur_cost_diff;
        for i in &self.edge_to_hisidx[&e] {
            let (_, actual_path_cost, cur_calculated_path_cost) = &self.history[*i];
            let new_calculated_path_cost = cur_calculated_path_cost - cur_edge_cost + new_edge_cost;
            new_diff += (actual_path_cost - new_calculated_path_cost).abs() - (actual_path_cost - cur_calculated_path_cost).abs();
        }
        new_diff
    }

    fn vertex(i: usize, j: usize) -> usize {
        i * W + j
    }
}

fn main() {
    let mut solver = Solver::new();
    solver.solve();
}
