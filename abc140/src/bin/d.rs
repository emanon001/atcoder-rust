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
        n: usize, mut k: usize,
        s: Chars
    };

    let other_ch = 'X';
    let mut sum = 0;
    let mut prev = other_ch;
    let mut count = 0;
    let mut section = 0;
    for ch in s.into_iter().chain(vec![other_ch].into_iter()) {
        if ch != prev {
            section += 1;
            if count > 1 {
                sum += count - 1;
            }
            count = 1;
        } else {
            count += 1;
        }
        prev = ch;
    }

    let m = std::cmp::min(section / 2, k);
    let res = std::cmp::min(sum + m * 2, n - 1);
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
