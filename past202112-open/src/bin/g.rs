#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn shortest_path(graph: &Vec<HashSet<usize>>, start: usize) -> Vec<Option<i64>> {
    let mut cost_list = vec![None; graph.len()];
    let mut que = std::collections::VecDeque::new();
    cost_list[start] = Some(0_i64);
    que.push_back(start);
    while let Some(u) = que.pop_front() {
        for &v in &graph[u] {
            if cost_list[v].is_some() {
                continue;
            }
            let new_cost = cost_list[u].unwrap() + 1_i64;
            cost_list[v] = Some(new_cost);
            que.push_back(v);
        }
    }
    cost_list
}

fn solve() {
    input! {
        n: usize, q: usize
    };

    let mut g = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            kind: usize, u: Usize1, v: Usize1,
        };
        if kind == 1 {
            if g[u].contains(&v) {
                g[u].remove(&v);
                g[v].remove(&u);
            } else {
                g[u].insert(v);
                g[v].insert(u);
            }
        } else {
            let res = if shortest_path(&g, u)[v].is_some() {
                "Yes"
            } else {
                "No"
            };
            println!("{}", res);
        }
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
