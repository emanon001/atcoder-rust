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
        _: usize, M: usize,
        edges: [(Usize1, Usize1); M],
    };

    let mut ans = 0;
    let mut edge_set = HashSet::new();
    for (u, v) in edges {
        if u == v {
            ans += 1;
            continue;
        }
        let edge = (u.min(v), u.max(v));
        if edge_set.contains(&edge) {
            ans += 1;
        } else {
            edge_set.insert(edge);
        }
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
