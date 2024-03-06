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
        S: Chars,
    };

    let mut indexes = HashMap::new();
    for (i, ch) in S.into_iter().enumerate() {
        let i = i + 1;
        indexes.entry(ch).or_insert(Vec::new()).push(i);
    }
    let ans = indexes
        .into_iter()
        .sorted_by_key(|(_, v)| v.len())
        .collect_vec()[0]
        .1[0];
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
