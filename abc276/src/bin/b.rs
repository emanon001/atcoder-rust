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
        n: usize, m: usize,
        edges: [(Usize1, Usize1); m]
    };

    let mut graph = vec![BTreeSet::new(); n];
    for (u, v) in edges {
        graph[u].insert(v + 1);
        graph[v].insert(u + 1);
    }
    for set in graph {
        let len = set.len();
        let res = if len == 0 {
            format!("{}", len)
        } else {
            format!("{} {}", len, set.iter().join(" "))
        };
        println!("{}", res);
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
