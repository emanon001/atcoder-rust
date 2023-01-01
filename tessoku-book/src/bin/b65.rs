#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize, t: Usize1,
        abv: [(Usize1, Usize1); n - 1]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in abv {
        graph[u].push(v);
        graph[v].push(u);
    }
    let mut dp = vec![0; n];
    dfs(t, 1 << 30, &graph, &mut dp);
    println!("{}", dp.iter().join(" "));
}

fn dfs(u: usize, parent: usize, graph: &[Vec<usize>], dp: &mut [usize]) -> usize {
    let mut res = 0;
    for &v in &graph[u] {
        if v == parent {
            continue;
        }
        chmax!(res, dfs(v, u, graph, dp) + 1);
    }
    dp[u] = res;
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
