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
        edges_t: [(Usize1, Usize1); m],
        edges_a: [(Usize1, Usize1); m],
    };

    let mut graph_t = vec![HashSet::new(); n];
    for (u, v) in edges_t {
        graph_t[u].insert(v);
        graph_t[v].insert(u);
    }

    for perm in (0..n).permutations(n) {
        let mut graph_a = vec![HashSet::new(); n];
        for &(u, v) in &edges_a {
            graph_a[perm[u]].insert(perm[v]);
            graph_a[perm[v]].insert(perm[u]);
        }
        if graph_t == graph_a {
            return println!("Yes");
        }
    }
    println!("No");
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
