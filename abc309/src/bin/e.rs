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
        n: usize, m: usize,
        pv: [Usize1; n - 1],
        xyv: [(Usize1, isize); m]
    };

    let mut graph = vec![Vec::new(); n];
    for u in 1..n {
        let p = pv[u - 1];
        graph[p].push(u);
        graph[u].push(p);
    }

    let mut max_y = vec![0; n];
    for (x, y) in xyv {
        chmax!(max_y[x], y + 1);
    }

    let res = dfs(0, 1 << 30, 0, &graph, &max_y);
    println!("{}", res);
}

fn dfs(u: usize, p: usize, rest_y: isize, graph: &Vec<Vec<usize>>, max_y: &Vec<isize>) -> usize {
    let mut res = 0;
    let rest_y = rest_y.max(max_y[u]);
    if rest_y > 0 {
        res += 1;
    }
    for &v in &graph[u] {
        if v == p {
            continue;
        }
        res += dfs(v, u, (rest_y - 1).max(0), graph, max_y);
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
