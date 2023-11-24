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
        n: usize, m: usize,
        s1: Chars,
        c: [usize; n]
    };

    let mut pos_map = HashMap::new();
    for (i, p) in c.into_iter().enumerate() {
        pos_map.entry(p).or_insert(Vec::new()).push(i);
    }
    let mut ans = vec![' '; n];
    for (c, pv) in pos_map {
        let m = pv.len();
        for (i, &p) in pv.iter().enumerate() {
            ans[pv[(i + 1) % m]] = s1[p];
        }
    }
    println!("{}", ans.iter().join(""));
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
