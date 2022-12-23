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
        x: i64, y: i64
    };

    let mut res = VecDeque::new();
    let mut tx = x;
    let mut ty = y;
    while tx != 1 || ty != 1 {
        res.push_front((tx, ty));
        if tx > ty {
            tx -= ty;
        } else {
            ty -= tx;
        }
    }
    println!("{}", res.len());
    println!(
        "{}",
        res.into_iter()
            .map(|(x, y)| format!("{} {}", x, y))
            .join("\n")
    );
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
