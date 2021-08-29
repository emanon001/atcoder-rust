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
        n: usize, k: i64,
        av: [i64; n]
    };

    let mut map = HashMap::new();
    for a in av {
        *map.entry(a).or_insert(0_i64) += 1;
    }
    let mut heap = BinaryHeap::new();
    for (a, c) in map {
        heap.push((a, c));
    }
    let mut rest = k;
    let mut res = 0_i64;
    while rest > 0 {
        let (a1, c1) = heap.pop().unwrap();
        if heap.is_empty() {
            let max_count = rest.min(a1 * c1);
            let diff = (max_count + c1 - 1) / c1;
            res += (a1 + 1) * a1 / 2 * c1
                - (a1 - diff + 1) * (a1 - diff) / 2 * c1;
            let sub_a = if max_count % c1 == 0 { 0 } else { c1 - (max_count % c1) };
            res -= (a1 - diff + 1) * sub_a;
            rest = 0;
        } else {
            let (a2, c2) = heap.pop().unwrap();
            let max_count = ((a1 - a2) * c1).min(rest);
            let diff = (max_count + c1 - 1) / c1;
            res += (a1 + 1) * a1 / 2 * c1
                - (a1 - diff + 1) * (a1 - diff) / 2 * c1;
            let sub_a = if max_count % c1 == 0 { 0 } else { c1 - (max_count % c1) };
            res -= (a1 - diff + 1) * sub_a;
            heap.push((a2, c2 + c1));
            rest -= max_count;
        }
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
