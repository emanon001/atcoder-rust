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
    };

    let ans = (1..=19)
        .combinations_with_replacement(3)
        .map(|v| {
            let d1 = v[0];
            let d2 = v[1];
            let d3 = v[2];
            "1".repeat(d1).parse::<i64>().unwrap()
                + "1".repeat(d2).parse::<i64>().unwrap()
                + "1".repeat(d3).parse::<i64>().unwrap()
        })
        .sorted()
        .nth(N - 1)
        .unwrap();
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
