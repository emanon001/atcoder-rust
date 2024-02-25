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
        N: usize, X: usize,
        edges: [(Usize1, Usize1, usize); N - 1],
    };

    let mut graph = vec![vec![]; N];
    for (u, v, c) in edges {
        graph[u].push((v, c));
        graph[v].push((u, c));
    }
    for u in 0..N {
        if is_reachable(u, 1 << 30, 0, X, &graph) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn is_reachable(u: usize, p: usize, d: usize, x: usize, graph: &Vec<Vec<(usize, usize)>>) -> bool {
    if d == x {
        return true;
    }
    if d > x {
        return false;
    }
    for &(v, c) in &graph[u] {
        if v == p {
            continue;
        }
        if is_reachable(v, u, d + c, x, graph) {
            return true;
        }
    }
    false
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
