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
        av: [Usize1; m]
    };

    let mut has_edge = vec![false; n];
    for a in av {
        has_edge[a] = true;
    }

    let mut ans = Vec::new();
    let mut stack = Vec::new();
    let mut u = 0;
    while u < n {
        stack.push(u);
        if !has_edge[u] {
            while let Some(u) = stack.pop() {
                ans.push(u + 1);
            }
            stack = Vec::new();
        }
        u += 1;
    }
    println!("{}", ans.iter().join(" "));
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
