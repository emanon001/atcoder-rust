#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize, M: usize, Q: usize,
        ABC: [(Usize1,Usize1, usize); M],
        X: [usize; Q],
    };

    let mut graph = vec![vec![]; N];
    for (a, b, c) in ABC {
        graph[a].push((b, c));
        graph[b].push((a, c));
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
