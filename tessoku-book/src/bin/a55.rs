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
        q: usize,
    };

    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! {
            kind: usize, x: isize
        };
        match kind {
            1 => {
                set.insert(x);
            }
            2 => {
                set.remove(&x);
            }
            3 => {
                let res = set.range(x..).next().unwrap_or(&-1);
                println!("{}", res);
            }
            _ => unreachable!(),
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
