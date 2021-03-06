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
        h: usize, w: usize, m: usize,
        hwv: [(Usize1, Usize1); m]
    };

    let mut sum_h = vec![0_i64; h];
    let mut sum_w = vec![0_i64; w];
    for &(i, j) in &hwv {
        sum_h[i] += 1;
        sum_w[j] += 1;
    }

    let max_h = *sum_h.iter().max().unwrap();
    let max_w = *sum_w.iter().max().unwrap();
    let max_hc = sum_h.iter().filter(|&s| s == &max_h).count();
    let max_wc = sum_w.iter().filter(|&s| s == &max_w).count();
    let sum = max_h + max_w;
    let on_bomb = hwv
        .into_iter()
        .filter(|&(h, w)| sum_h[h] + sum_w[w] == sum)
        .count()
        == max_hc * max_wc;
    let res = if on_bomb { sum - 1 } else { sum };
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
