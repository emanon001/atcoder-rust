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
        lv: [usize; n]
    };

    let res = lv
        .into_iter()
        .combinations(3)
        .filter(|v| {
            let mut v = v.clone();
            v.sort();
            v[0] + v[1] > v[2] && v.iter().collect::<HashSet<_>>().len() == 3
        })
        .count();
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
