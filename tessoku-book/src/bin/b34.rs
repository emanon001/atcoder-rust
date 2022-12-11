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
        n: usize, x: usize, y: usize,
        av: [usize; n]
    };

    let max = 5;
    let mut dp = vec![0; max];
    let moves = vec![x, y];
    for i in 0..max {
        let mut transit = vec![false; moves.len() + 1];
        for &m in &moves {
            if i >= m {
                transit[dp[i - m]] = true;
            }
        }
        let (g, _) = transit.iter().find_position(|&x| !x).unwrap();
        dp[i] = g;
    }
    let xor = av.into_iter().fold(0, |acc, a| acc ^ dp[a % max]);
    let res = if xor == 0 { "Second" } else { "First" };
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
