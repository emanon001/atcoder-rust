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
        n: u64,
        edges: [(usize, usize); n - 1]
    };

    let mut vertex_count = 0_u64;
    for c in 1..=n {
        vertex_count += c * (n - c + 1);
    }
    let mut edge_count = 0_u64;
    for (u, v) in edges {
        let l = u.min(v);
        let r = u.max(v);
        edge_count += l as u64 * (n - r as u64 + 1) as u64;
    }
    let res = vertex_count - edge_count;
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
