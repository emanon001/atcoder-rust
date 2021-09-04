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
        l: usize, q: usize,
        cxv: [(usize, usize); q]
    };

    let mut set: BTreeSet<usize> = BTreeSet::new();
    for (c, x) in cxv {
        match c {
            1 => {
                set.insert(x);
            }
            2 => {
                let a = set.range(..x).next_back().unwrap_or(&0);
                let b = set.range(x..).next().unwrap_or(&l);
                let res = b - a;
                println!("{}", res);
            }
            _ => unreachable!()
        }
    }
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
