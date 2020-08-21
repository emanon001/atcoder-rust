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
        n: usize, k: usize,
        xv: [usize; n]
    };

    let mut heap = xv[..k]
        .into_iter()
        .enumerate()
        .map(|(i, x)| (*x, i + 1))
        .collect::<BinaryHeap<_>>();

    println!("{}", heap.peek().unwrap().1);
    for i in k..n {
        let x = xv[i];
        let i = i + 1;
        if let Some((y, j)) = heap.pop() {
            if x < y {
                heap.push((x, i));
            } else {
                heap.push((y, j));
            }
        }
        println!("{}", heap.peek().unwrap().1);
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
