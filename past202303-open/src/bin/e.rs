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
        H: usize, _: usize,
        S: [Chars; H],
        T: [Chars; H],
    };

    for (r1, r2) in S.into_iter().zip(T) {
        let mut counts1 = HashMap::new();
        let mut counts2 = HashMap::new();
        for ch in r1 {
            *counts1.entry(ch).or_insert(0) += 1;
        }
        for ch in r2 {
            *counts2.entry(ch).or_insert(0) += 1;
        }
        if counts1 != counts2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
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
