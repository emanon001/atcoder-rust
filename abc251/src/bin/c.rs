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
        stv: [(String, i64); n]
    };

    let mut set = HashSet::new();
    let mut orig_list = Vec::new();
    for (i, (s, t)) in stv.into_iter().enumerate() {
        if set.contains(&s) {
            continue;
        }
        set.insert(s);
        orig_list.push((t, i + 1));
    }
    orig_list.sort_by_key(|o| (-o.0, o.1));
    println!("{}", orig_list[0].1);
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
