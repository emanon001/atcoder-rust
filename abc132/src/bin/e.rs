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
    v: usize,
}

type Edge = (usize, usize);
impl Graph {
    pub fn new(edges: &[Edge], v: usize) -> Self {
        let mut graph = vec![Vec::new(); v];
        for &(u, v) in edges {
            graph[u].push(v);
            graph[v].push(u);
        }
        Self { graph, v }
    }

    pub fn new_directed(edges: &[Edge], v: usize) -> Self {
        let mut graph = vec![Vec::new(); v];
        for &(u, v) in edges {
            graph[u].push(v);
        }
        Self { graph, v }
    }

    pub fn shortest_path(&self, start: usize) -> Vec<Option<isize>> {
        // u, step
        let mut cost_list = vec![vec![None; 3]; self.v];
        let mut que = std::collections::VecDeque::new();
        cost_list[start][2] = Some(0);
        // (u, step)
        que.push_back((start, 2));
        while let Some((u, step)) = que.pop_front() {
            let new_step = (step + 1) % 3;
            let cost = cost_list[u][step].unwrap();
            let new_cost = if new_step == 2 { cost + 1 } else { cost };
            for &v in &self.graph[u] {
                if cost_list[v][new_step].is_some() {
                    continue;
                }
                cost_list[v][new_step] = Some(new_cost);
                que.push_back((v, new_step));
            }
        }
        cost_list.into_iter().map(|v| v[2]).collect::<Vec<_>>()
    }
}

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m],
        s: Usize1, t: Usize1
    };

    let graph = Graph::new_directed(&edges, n);
    let res = graph.shortest_path(s)[t].unwrap_or(-1);
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
