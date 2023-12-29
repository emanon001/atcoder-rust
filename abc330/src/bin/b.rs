#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize, l: i64, r: i64,
        a: [i64; n]
    };

    let ans = a
        .into_iter()
        .map(|a_i| {
            if (l..=r).contains(&a_i) {
                a_i
            } else if (l - a_i).abs() <= (r - a_i).abs() {
                l
            } else {
                r
            }
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
