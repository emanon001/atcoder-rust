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
        A: [u64; N],
    };

    let mut counts = HashMap::new();
    for &a in &A {
        *counts.entry(a).or_insert(0_u64) += 1;
    }
    let mut ans = 0_u64;
    for (&a, &c) in &counts {
        let b = 100000 - a;
        ans += if a == b {
            c * (c - 1)
        } else if let Some(c2) = counts.get(&b) {
            c * c2
        } else {
            0
        }
    }
    ans /= 2;
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
