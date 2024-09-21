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
        N: usize, K: usize,
        mut AB: [(u64, u64); N],
    };

    let mut v = Vec::new();
    for (a, b) in AB {
        v.push(b);
        v.push(a - b);
    }
    v.sort_by_key(|&x| std::cmp::Reverse(x));
    let ans = v.into_iter().take(K).sum::<u64>();
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
