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
        n: usize,
        edges: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for &(u, v) in &edges {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut colors = HashMap::new();
    let mut que = VecDeque::new();
    let not_found = std::usize::MAX;
    // (u, parent)
    que.push_back((0, not_found));
    while let Some((u, parent)) = que.pop_front() {
        let pcolor = *colors.get(&(parent, u)).unwrap_or(&not_found);
        let mut color = 1;
        for &v in &graph[u] {
            if v == parent {
                continue;
            }
            if color == pcolor {
                color += 1;
            }
            colors.insert((u, v), color);
            que.push_back((v, u));
            color += 1;
        }
    }
    println!("{}", colors.values().max().unwrap());
    for e in edges {
        println!("{}", colors.get(&e).unwrap());
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
