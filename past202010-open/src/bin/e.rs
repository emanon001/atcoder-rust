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
        N: usize,
        S: Chars,
    };

    for orders in (0..N).permutations(N) {
        let t = orders.iter().map(|&i| S[i]).collect_vec();
        if t != S && t.iter().copied().rev().collect_vec() != S {
            println!("{}", t.iter().join(""));
            return;
        }
    }
    println!("None");
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
