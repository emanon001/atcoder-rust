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

struct Solver {
    graph: Vec<Vec<(usize, i64, char)>>
}

impl Solver {
    fn new() -> Self {
        let n = H * W;
        let mut graph = vec![Vec::new(); n];
        let cost = 1;
        for i in 0..H {
            for j in 0..W {
                let u = Self::vertex(i, j);
                // ↑
                if i > 0 {
                    let v = Self::vertex(i - 1, j);
                    graph[u].push((v, cost, 'U'));
                }
                // ↓
                if i + 1 < H {
                    let v = Self::vertex(i + 1, j);
                    graph[u].push((v, cost, 'D'));
                }
                // ←
                if j > 0 {
                    let v = Self::vertex(i, j - 1);
                    graph[u].push((v, cost, 'L'));
                }
                // →
                if j + 1 < W {
                    let v = Self::vertex(i, j + 1);
                    graph[u].push((v, cost, 'R'));
                }
            }
        }
        Self {
            graph
        }
    }

    fn solve(&mut self) {
        for _ in 0..1000 {
            let (si, sj, ti, tj): (usize, usize, usize, usize) = parse_line().unwrap();
            let d = self.shortest_path(Self::vertex(si, sj));
            println!("{}", d[Self::vertex(ti, tj)].iter().join(""));
            let _: i64 = parse_line().unwrap();
        }
    }

    fn shortest_path(&self, start: usize) -> Vec<Vec<char>> {
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
            for &(v, w, d) in &self.graph[u] {
                let new_cost = cost + w;
                if new_cost < cost_list[v] {
                    let mut new_path= path.clone();
                    new_path.push(d);
                    path_list[v] = new_path.clone();
                    cost_list[v] = new_cost;
                    heap.push(std::cmp::Reverse((new_cost, v, new_path)));
                }
            }
        }
        path_list
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
