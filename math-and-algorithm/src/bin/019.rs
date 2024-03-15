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
        A: [Usize1; N],
    };

    let mut counts = vec![0; 3];
    for &a in &A {
        counts[a] += 1;
    }
    let ans = counts
        .into_iter()
        .map(|c| if c >= 2 { c * (c - 1) / 2 } else { 0 })
        .sum::<usize>();
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
