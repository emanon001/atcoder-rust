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
        sv: [Chars; n]
    };

    for perm in (0..n).permutations(n) {
        let is_ok = (0..n - 1).all(|i| {
            (0..m).any(|wildcard_pos| {
                (0..m).all(|j| wildcard_pos == j || sv[perm[i]][j] == sv[perm[i + 1]][j])
            })
        });
        if is_ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
