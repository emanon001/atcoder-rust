#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, M: usize,
        edges: [(usize, usize); M]
    };

    let mut used = HashSet::new();
    for (u, v) in edges {
        if u > N || v > N {
            println!("No");
            return;
        }
        if u == v {
            println!("No");
            return;
        }
        let edge = (u.min(v), u.max(v));
        if used.contains(&edge) {
            println!("No");
            return;
        }
        used.insert(edge);
    }
    println!("Yes");
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
