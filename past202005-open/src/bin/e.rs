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
        n: usize, m: usize, q: usize,
        edges: [(Usize1, Usize1); m],
        mut colors: [usize; n]
    };

    let mut graph = vec![Vec::new(); n];
    for (u, v) in edges {
        graph[u].push(v);
        graph[v].push(u);
    }

    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: Usize1,
                };
                println!("{}", colors[x]);
                for &u in &graph[x] {
                    colors[u] = colors[x];
                }
            }
            2 => {
                input! {
                    x: Usize1, y: usize
                };
                println!("{}", colors[x]);
                colors[x] = y;
            }
            _ => unreachable!(),
        };
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
