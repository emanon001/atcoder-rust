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
        n: usize, x: usize, y: usize, z: usize,
        av: [usize; n],
        bv: [usize; n],
    };

    let scores = av
        .into_iter()
        .zip(bv)
        .enumerate()
        .map(|(i, (a, b))| (a, b, a + b, i))
        .collect::<Vec<_>>();

    let mut gokaku_set = BTreeSet::new();
    for s in scores.iter().sorted_by(|&a, &b| b.0.cmp(&a.0)) {
        let i = s.3 + 1;
        if gokaku_set.len() < x {
            gokaku_set.insert(i);
        }
    }
    for s in scores.iter().sorted_by(|&a, &b| b.1.cmp(&a.1)) {
        let i = s.3 + 1;
        if gokaku_set.len() < x + y && !gokaku_set.contains(&i) {
            gokaku_set.insert(i);
        }
    }
    for s in scores
        .iter()
        .sorted_by(|&a, &b| b.2.cmp(&a.2))
        .take(x + y + z)
    {
        let i = s.3 + 1;
        if gokaku_set.len() < x + y + z && !gokaku_set.contains(&i) {
            gokaku_set.insert(i);
        }
    }

    println!("{}", gokaku_set.iter().join("\n"));
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
