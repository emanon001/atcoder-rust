#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![vec![false; n]; n];
    for (u, v) in edges {
        graph[u][v] = true;
        graph[v][u] = true;
    }
    let mut res = 0;
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if graph[a][b] && graph[b][c] && graph[c][a] {
                    res += 1;
                }
            }
        }
    }
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
