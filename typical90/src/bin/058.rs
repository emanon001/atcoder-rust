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
        N: u64, K: u64,
    };

    let _mod = 10_u64.pow(5);

    let mut table = vec![0; 10.pow(5) + 1];
    for n in 0..table.len() {
        let m = (n as u64
            + n.to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u64)
                .sum::<u64>())
            % _mod;
        table[n] = m;
    }

    let mut doubling = vec![vec![0; table.len()]; 60];
    doubling[0] = table.clone();
    for i in 1..doubling.len() {
        for j in 0..table.len() {
            doubling[i][j] = doubling[i - 1][doubling[i - 1][j] as usize];
        }
    }

    let mut k = K;
    let mut ans = N;
    while k > 0 {
        let mut i = 0;
        while 2.pow(i + 1) < k {
            i += 1;
            if i > 60 {
                eprintln!("{}", i);
            }
        }
        ans = doubling[i as usize][ans as usize];
        k -= 2.pow(i);
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
