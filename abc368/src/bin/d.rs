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
        N: usize, K: usize,
        AB: [(Usize1, Usize1); N - 1],
        V: [Usize1; K],
    };

    let mut graph = vec![vec![]; N];
    for (a, b) in AB {
        graph[a].push(b);
        graph[b].push(a);
    }
    let u = V[0];
    let set = V.into_iter().collect::<HashSet<_>>();
    let ans = dfs(u, N, &graph, &set);
    println!("{}", ans);
}

fn dfs(u: usize, p: usize, g: &Vec<Vec<usize>>, set: &HashSet<usize>) -> usize {
    let mut res = 0;
    for &v in &g[u] {
        if v == p {
            continue;
        }
        res += dfs(v, u, g, set);
    }
    if res > 0 || set.contains(&u) {
        res += 1;
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
