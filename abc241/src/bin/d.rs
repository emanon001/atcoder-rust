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
    let mut id = 1;
    let mut set = BTreeSet::new();
    for _ in 0..q {
        input! { kind: usize }
        match kind {
            1 => {
                input! {
                    x: i64
                }
                set.insert((x, id));
                id += 1;
            }
            2 => {
                input! {
                    x: i64, k: usize
                }
                let mut iter = set.range(..=(x, 10.pow(9)));
                for _ in 0..k - 1 {
                    iter.next_back();
                }
                let res = if let Some(res) = iter.next_back() {
                    res.0
                } else {
                    -1_i64
                };
                println!("{}", res);
            }
            3 => {
                input! {
                    x: i64, k: usize
                }
                let mut iter = set.range((x, 0)..);
                for _ in 0..k - 1 {
                    iter.next();
                }
                let res = if let Some(res) = iter.next() {
                    res.0
                } else {
                    -1_i64
                };
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
