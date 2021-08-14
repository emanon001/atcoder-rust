#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        sv: [u64; n],
        tv: [u64; n],
    };

    let mut heap = BinaryHeap::new();
    for i in 0..n {
        let t = tv[i];
        heap.push(std::cmp::Reverse((t, i)));
    }
    let mut res = vec![1_u64 << 60; n];
    while let Some(std::cmp::Reverse((a, i))) = heap.pop() {
        if res[i] >= a {
            res[i] = a;
            let j = (i + 1) % n;
            if chmin!(res[j], res[i] + sv[i]) {
                heap.push(std::cmp::Reverse((res[j], j)));
            }
        }
    }
    for i in 0..n {
        println!("{}", res[i]);
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
