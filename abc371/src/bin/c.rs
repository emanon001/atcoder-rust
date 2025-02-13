#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        M: usize,
        edges: [(Usize1, Usize1); M],
        M2: usize,
        edges2: [(Usize1, Usize1); M2],
    };

    let mut A = vec![];
    for i in 0..N - 1 {
        input_interactive! {
            a: [i64; N - i - 1],
        };
        A.push(a);
    }

    let mut ans = 1_i64 << 60;
    for perm in (0..N).permutations(N) {
        let mut cost = 0;
        // 辺の追加
        for &(u, v) in &edges {
            let u2 = perm[u].min(perm[v]);
            let v2 = perm[u].max(perm[v]);
            let exists = edges2.iter().any(|&(a, b)| a == u2 && b == v2);
            if !exists {
                cost += A[u2][v2 - u2 - 1];
            }
        }
        // 辺の削除
        for &(u, v) in &edges2 {
            let exists = edges.iter().any(|&(a, b)| {
                let u2 = perm[a].min(perm[b]);
                let v2 = perm[a].max(perm[b]);
                u == u2 && v == v2
            });
            if !exists {
                cost += A[u][v - u - 1];
            }
        }
        chmin!(ans, cost);
    }
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
