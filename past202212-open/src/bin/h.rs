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
        _: usize,
        N: Chars,
    };

    let mut counts = vec![0_i64; 10];
    for ch in N {
        let d = ch.to_digit(10).unwrap() as usize;
        counts[d] += 1;
    }
    let mut ans = 0;
    for x in 0..9 {
        for y in x + 1..=9 {
            ans += (x - y).abs() as i64 * counts[x as usize] * counts[y as usize];
        }
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
