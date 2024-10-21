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
        N: usize, M: usize,
        edges: [(Usize1, Usize1); M],
    };

    let mut graph = vec![vec![]; N];
    for (u, v) in edges {
        graph[u].push(v);
    }

    let mut que = VecDeque::new();
    que.push_back((0, 0));
    let mut visited = vec![false; N];
    while let Some((u, count)) = que.pop_front() {
        if visited[u] {
            if u == 0 {
                println!("{}", count);
                return;
            }
            continue;
        }
        visited[u] = true;
        for &v in &graph[u] {
            que.push_back((v, count + 1));
        }
    }
    println!("-1");
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
