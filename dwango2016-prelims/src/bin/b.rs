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
        kv: [usize; n - 1]
    };

    let mut res = kv
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.min(b))
        .collect::<VecDeque<_>>();
    res.push_front(&kv[0]);
    res.push_back(&kv[kv.len() - 1]);
    let res = res.into_iter().join(" ");
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
