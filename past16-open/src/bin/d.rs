#[allow(unused_imports)]
use itertools::Itertools;
use num::rational::Rational64;
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
        A: [i64; N]
    };

    let max = *A.iter().max().unwrap();
    let ans = A
        .into_iter()
        .map(|a| {
            (Rational64::new(a, max) * Rational64::new(10.pow(9), 1))
                .round()
                .to_integer()
        })
        .join(" ");
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
