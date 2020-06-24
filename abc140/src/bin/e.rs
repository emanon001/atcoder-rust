#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)] // for `input! { N: ... }`
fn solve() {
    input! {
        N: usize,
        P: [usize; N]
    };

    let mut indexes = vec![0; N + 1];
    for (i, &p) in P.iter().enumerate() {
        indexes[p] = i;
    }
    let mut set: BTreeSet<usize> = BTreeSet::new();
    set.insert(indexes[N]);
    let mut res = 0_usize;
    for n in (1..N).rev() {
        let i = indexes[n];
        let mut iter = set.range(0..i);
        let l1 = iter.next_back();
        let l2 = iter.next_back();
        let mut iter = set.range(i + 1..);
        let r1 = iter.next();
        let r2 = iter.next();
        res += match (l2, l1, r1) {
            (None, Some(l1), None) => (l1 + 1) * (N - i) * n,
            (None, Some(l1), Some(r1)) => (l1 + 1) * (r1 - i) * n,
            (Some(l2), Some(l1), None) => (l1 - l2) * (N - i) * n,
            (Some(l2), Some(l1), Some(r1)) => (l1 - l2) * (r1 - i) * n,
            _ => 0,
        };
        res += match (l1, r1, r2) {
            (None, Some(r1), None) => (i + 1) * (N - r1) * n,
            (None, Some(r1), Some(r2)) => (i + 1) * (r2 - r1) * n,
            (Some(l1), Some(r1), None) => (i - l1) * (N - r1) * n,
            (Some(l1), Some(r1), Some(r2)) => (i - l1) * (r2 - r1) * n,
            _ => 0,
        };
        set.insert(i);
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
