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
        n: usize, c: i64,
        abcv: [(usize, usize, i64); n]
    };
    let mut idexes = BTreeSet::new();
    let mut plus = HashMap::new();
    for &(a, b, c) in &abcv {
        idexes.insert(a);
        idexes.insert(b + 1);
        *plus.entry(a).or_insert(0) += c;
        *plus.entry(b + 1).or_insert(0) -= c;
    }
    let mut res = 0;
    let mut cur_idx = 1;
    let mut cur_cost = 0;
    for i in idexes {
        res += (i - cur_idx) as i64 * cur_cost.min(c);
        cur_cost += plus[&i];
        cur_idx = i;
    }
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
