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
        n: usize,
        abv: [(u64, u64); n]
    };

    let res = abv
        .into_iter()
        .enumerate()
        .sorted_by(|(i, (a, b)), (j, (a2, b2))| {
            let lcm = (a + b).lcm(&(a2 + b2));
            (lcm / (a2 + b2) * a2)
                .cmp(&(lcm / (a + b) * a))
                .then(i.cmp(j))
        })
        .map(|(i, _)| i + 1)
        .join(" ");
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
