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
        N: usize, S: usize, T: usize,
    };

    let win = match (S, T) {
        (0, 0) => N.is_odd(),
        (0, 1) => N.is_even(),
        (1, 0) => N.is_even(),
        (1, 1) => N.is_odd(),
        _ => unreachable!(),
    };
    let ans = if win { "Alice" } else { "Bob" };
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
