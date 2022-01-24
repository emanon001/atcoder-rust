#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        av: [i64; n],
        bv: [i64; n],
    };

    let mut points = av
        .into_iter()
        .zip(bv.into_iter())
        .enumerate()
        .collect::<Vec<_>>();
    points.sort_by_key(|(i, (a, b))| (Reverse(*a + *b), Reverse(*a), *i));
    println!("{}", points.into_iter().map(|(i, _)| i + 1).join(" "));
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
