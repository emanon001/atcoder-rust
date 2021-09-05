#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;
use std::cmp::Reverse;

fn solve() {
    input! {
        q: usize,
    };

    let mut front_heap = BinaryHeap::new();
    let mut back = VecDeque::new();
    for _ in 0..q {
        input! { kind: usize };
        match kind {
            1 => {
                input! { x: usize };
                back.push_back(x);
            }
            2 => {
                if !front_heap.is_empty() {
                    let Reverse(v) = front_heap.pop().unwrap();
                    println!("{}", v);
                } else {
                    let v = back.pop_front().unwrap();
                    println!("{}", v);
                }
            }
            3 => {
                for &v in &back {
                    front_heap.push(Reverse(v));
                }
                back = VecDeque::new();
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
