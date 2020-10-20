#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(u: usize, graph: &[Vec<usize>]) -> usize {
    if graph[u].is_empty() {
        return 1;
    }
    let mut max = 0;
    let mut min = std::usize::MAX;
    for &v in &graph[u] {
        let x = f(v, graph);
        max = max.max(x);
        min = min.min(x);
    }
    max + min + 1
}

fn solve() {
    input! {
        n: usize,
        bv: [Usize1; n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for u in 1..n {
        let p = bv[u - 1];
        graph[p].push(u);
    }
    let res = f(0, &graph);
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
