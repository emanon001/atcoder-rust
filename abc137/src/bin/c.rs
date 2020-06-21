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
        sv: [Chars; n]
    };

    let mut table = HashMap::new();
    for s in sv {
        let mut s = s;
        s.sort();
        let s = s.into_iter().rev().collect::<String>();
        *table.entry(s).or_insert(0) += 1;
    }
    let mut res = 0_u64;
    for (_, c) in table {
        res += c * (c - 1) / 2;
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
