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
        av: [usize; n]
    };

    let iter = av.into_iter().map(|a| a - 1);
    let cusum = vec![0]
        .into_iter()
        .chain(iter)
        .scan(0, |acc, a| {
            *acc = (*acc + a) % k;
            Some(*acc)
        })
        .collect::<Vec<_>>();
    let mut counts = HashMap::new();
    let mut que = VecDeque::new();
    let mut res = 0_u64;
    for r in 0..n + 1 {
        let a = cusum[r];
        res += counts.get(&a).unwrap_or(&0);
        *counts.entry(a).or_insert(0) += 1;
        que.push_back(a);
        if que.len() == k {
            let b = que.pop_front().unwrap();
            *counts.entry(b).or_insert(1) -= 1;
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
