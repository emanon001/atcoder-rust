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
        mut s: Chars,
        n: usize,
        lrv: [(Usize1, Usize1); n]
    };

    for (l, r) in lrv {
        let front = s.iter().take(l).collect::<Vec<_>>();
        let mid = s[l..=r].into_iter().rev().collect::<Vec<_>>();
        let back = s.iter().skip(r + 1).collect::<Vec<_>>();
        s = front
            .into_iter()
            .chain(mid)
            .chain(back)
            .copied()
            .collect::<Vec<char>>();
    }
    println!("{}", s.into_iter().collect::<String>());
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
