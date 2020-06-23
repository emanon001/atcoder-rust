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
    pub fn new_directed(edges: &[Edge], v: usize) -> Self {
        let mut graph = vec![Vec::new(); v];
        for &(u, v) in edges {
            graph[u].push(v);
        }
        Graph { graph, v }
    }

    pub fn solve(&self, mut indegreev: Vec<usize>) -> isize {
        let mut dp = vec![0; self.v];
        let mut que = VecDeque::new();
        for u in 0..self.v {
            if indegreev[u] == 0 {
                que.push_back(u);
                dp[u] = 1;
            }
        }
        while let Some(u) = que.pop_front() {
            for &v in &self.graph[u] {
                indegreev[v] -= 1;
                if indegreev[v] == 0 {
                    que.push_back(v);
                    dp[v] = dp[u] + 1;
                }
            }
        }

        let is_ok = indegreev.into_iter().all(|x| x == 0);
        if is_ok {
            dp.into_iter().max().unwrap_or(0) as isize
        } else {
            -1
        }
    }
}

fn solve() {
    input! {
        n: usize,
        matchv: [[Usize1; n - 1]; n]
    };

    let vn = n * (n - 1) / 2;
    let id_table = (0..n)
        .into_iter()
        .combinations(2)
        .enumerate()
        .map(|(i, ab)| ((ab[0].min(ab[1]), ab[0].max(ab[1])), i))
        .collect::<HashMap<_, _>>();
    let mut indegreev = vec![0; vn];
    let mut edges = Vec::new();
    for i in 0..n {
        let a = i;
        for mv in matchv[i].windows(2) {
            let b = mv[0];
            let u = *id_table.get(&(a.min(b), a.max(b))).unwrap();
            let c = mv[1];
            let v = *id_table.get(&(a.min(c), a.max(c))).unwrap();
            indegreev[v] += 1;
            edges.push((u, v));
        }
    }

    let graph = Graph::new_directed(&edges, vn);
    let res = graph.solve(indegreev);
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
