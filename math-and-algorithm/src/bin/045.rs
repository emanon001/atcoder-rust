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
        AB: [(Usize1, Usize1); M],
    };

    let mut counts = vec![0; N];
    for (u, v) in AB {
        if u > v {
            counts[u] += 1;
        } else if v > u {
            counts[v] += 1;
        }
    }
    let ans = counts.into_iter().filter(|&c| c == 1).count();
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
