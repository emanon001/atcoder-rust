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
    let ans = if is_bipartite_graph(N, &graph) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}

fn is_bipartite_graph(n: usize, graph: &[Vec<usize>]) -> bool {
    let mut colors = vec![-1; n];
    let mut res = true;
    for u in 0..n {
        if colors[u] != -1 {
            continue;
        }
        res = res && traverse(u, 0, &mut colors, graph);
    }
    res
}

fn traverse(u: usize, color: isize, colors: &mut Vec<isize>, graph: &[Vec<usize>]) -> bool {
    if colors[u] != -1 {
        return colors[u] == color;
    }
    colors[u] = color;
    let next_color = (color + 1) % 2;
    let mut res = true;
    for &v in &graph[u] {
        res = res && traverse(v, next_color, colors, graph);
    }
    res
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
