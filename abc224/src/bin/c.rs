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
        points: [(i64, i64); n]
    };

    let mut res = 0_u64;
    for points in points.into_iter().combinations(3) {
        let p1 = points[0];
        let p2 = points[1];
        let p3 = points[2];
        let mut set = HashSet::new();
        set.insert(p1);
        set.insert(p2);
        set.insert(p3);
        if set.len() < 3 {
            continue;
        }
        let area =
            p1.0 * p2.1 + p2.0 * p3.1 + p3.0 * p1.1 - p1.1 * p2.0 - p2.1 * p3.0 - p3.1 * p1.0;
        if area != 0 {
            res += 1;
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
