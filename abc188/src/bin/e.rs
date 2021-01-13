#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(u: usize, graph: &[Vec<usize>], av: &[i64], dp: &mut Vec<Option<i64>>) -> i64 {
    if let Some(res) = dp[u] {
        return res;
    }
    let mut res = std::i64::MIN;
    for &v in &graph[u] {
        let d = av[v].max(f(v, graph, av, dp));
        if d > res {
            res = d;
        }
    }
    dp[u] = Some(res);
    return res;
}

fn solve() {
    input! {
        n: usize, m: usize,
        av: [i64; n],
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
    }
    let mut dp = vec![None; n];
    let mut res = std::i64::MIN;
    for u in 0..n - 1 {
        let a = av[u];
        let b = f(u, &graph, &av, &mut dp);
        if b == std::i64::MIN {
            continue;
        }
        let c = b - a;
        if c > res {
            res = c;
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
