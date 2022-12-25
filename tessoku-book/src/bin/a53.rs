#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        q: usize,
    };

    let mut heap = BinaryHeap::new();
    for _ in 0..q {
        input! { kind: usize };
        match kind {
            1 => {
                input! { x: usize };
                heap.push(Reverse(x));
            }
            2 => {
                println!("{}", heap.peek().unwrap().0);
            }
            3 => {
                heap.pop();
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
