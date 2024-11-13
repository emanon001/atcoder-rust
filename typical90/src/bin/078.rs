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
        edges: [(Usize1, Usize1); M],
    };

    let mut graph = vec![vec![]; N];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    let ans = (0..N)
        .map(|u| graph[u].iter().filter(|&v| v < &u).count())
        .filter(|&c| c == 1)
        .count();
    println!("{}", ans);
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
