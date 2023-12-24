#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize, m: usize,
        a: [Usize1; m],
        b: [Usize1; m],
    };

    let mut graph = vec![vec![]; n];
    for i in 0..m {
        let u = a[i];
        let v = b[i];
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut colors = vec![None; n];
    for u in 0..n {
        if colors[u].is_some() {
            continue;
        }
        colors[u] = Some(0);
        if !dfs(u, &graph, &mut colors) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn dfs(u: usize, graph: &[Vec<usize>], colors: &mut [Option<usize>]) -> bool {
    let color = colors[u].unwrap();
    for &v in &graph[u] {
        let new_color = (color + 1) % 2;
        if let Some(c) = colors[v] {
            if c != new_color {
                return false;
            }
        } else {
            colors[v] = Some(new_color);
            if !dfs(v, graph, colors) {
                return false;
            }
        }
    }
    true
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
