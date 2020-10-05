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
        mut abv: [(usize, usize); n]
    };

    abv.sort();
    let mut abv = abv.into_iter().collect::<VecDeque<_>>();
    let mut heap = BinaryHeap::new();
    let mut res = 0;
    for i in 1..n + 1 {
        while let Some(ab) = abv.front() {
            if ab.0 > i {
                break;
            }
            heap.push(ab.1);
            abv.pop_front();
        }
        res += heap.pop().unwrap();
        println!("{}", res);
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
