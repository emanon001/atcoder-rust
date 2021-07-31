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

    let mut heap = BinaryHeap::new();
    let mut add = 0_i64;
    for _ in 0..q {
        input! {
            kind: usize,
        };
        match kind {
            1 => {
                input! {
                    x: i64
                };
                let x = x - add;
                heap.push(std::cmp::Reverse(x));
            }
            2 => {
                input! {
                    x: i64
                };
                add += x;
            }
            3 => {
                let std::cmp::Reverse(x) = heap.pop().unwrap();
                let res = x + add;
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
