#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        AB: [(Usize1, Usize1); M],
    };

    let mut graph = vec![vec![]; N];
    for (u, v) in AB {
        graph[u].push(v);
        graph[v].push(u);
    }
    let ans = shortest_path(&graph).into_iter().join(" ");
    println!("{}", ans);
}

fn shortest_path(graph: &Vec<Vec<usize>>) -> Vec<i64> {
    let mut dist_list = vec![-1; graph.len()];
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    while let Some((u, d)) = que.pop_front() {
        if dist_list[u] != -1 {
            continue;
        }
        dist_list[u] = d;
        for &v in &graph[u] {
            if dist_list[v] == -1 {
                que.push_back((v, d + 1));
            }
        }
    }
    dist_list
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
