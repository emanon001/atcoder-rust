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
        ABC: [(Usize1, Usize1, i64); M],
    };

    let mut graph = vec![vec![]; N];
    for (a, b, c) in ABC {
        graph[a].push((b, c));
        graph[b].push((a, c));
    }

    let mut ans = -(1_i64 << 60);
    for u in 0..N {
        let mut visited = HashSet::new();
        visited.insert(u);
        ans = ans.max(dfs(u, &graph, &mut visited));
    }
    println!("{}", ans);
}

fn dfs(u: usize, graph: &[Vec<(usize, i64)>], visited: &mut HashSet<usize>) -> i64 {
    let mut res = 0;
    for (v, d) in &graph[u] {
        if visited.contains(v) {
            continue;
        }
        visited.insert(*v);
        res = res.max(dfs(*v, graph, visited) + d);
        visited.remove(v);
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
