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

    let mut graph = vec![BTreeSet::new(); n];
    for (u, v) in edges {
        graph[u].insert(v);
        graph[v].insert(u);
    }
    let mut first = vec![n; n];
    let mut res = Vec::new();
    let mut cur = 0;
    loop {
        res.push(cur + 1);
        if graph[cur].is_empty() {
            if cur == 0 {
                break;
            }
            cur = first[cur];
        } else {
            let u = *graph[cur].iter().next().unwrap();
            graph[cur].remove(&u);
            graph[u].remove(&cur);
            if first[u] == n {
                first[u] = cur;
            }
            cur = u;
        }
    }
    println!("{}", res.iter().join(" "));
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
