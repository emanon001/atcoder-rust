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
        xyv: [(i64, i64); n]
    };

    let mut ixyv = xyv.iter().copied().enumerate().collect::<Vec<_>>();
    let mut set = HashSet::new();
    ixyv.sort_by_key(|(_, a)| a.0);
    set.insert((ixyv[0].0, ixyv[n - 1].0));
    set.insert((ixyv[0].0, ixyv[n - 2].0));
    set.insert((ixyv[1].0, ixyv[n - 1].0));
    ixyv.sort_by_key(|(_, a)| a.1);
    set.insert((ixyv[0].0, ixyv[n - 1].0));
    set.insert((ixyv[0].0, ixyv[n - 2].0));
    set.insert((ixyv[1].0, ixyv[n - 1].0));
    let mut v = Vec::new();
    for (i, j) in set {
        v.push((xyv[i].0 - xyv[j].0).abs().max((xyv[i].1 - xyv[j].1).abs()));
    }
    v.sort_by_key(|x| -x);
    println!("{}", v[1]);
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
