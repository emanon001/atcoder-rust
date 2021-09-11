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
        points: [(usize, usize); n]
    };

    let mut set = HashSet::new();
    for &p in &points {
        set.insert(p);
    }
    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            let p1 = points[i];
            let p2 = points[j];
            if p1.0 == p2.0 || p1.1 == p2.1 {
                continue;
            }
            let p3 = (p1.0, p2.1);
            let p4 = (p2.0, p1.1);
            if set.contains(&p3) && set.contains(&p4) {
                res += 1;
            }
        }
    }
    println!("{}", res / 2);
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
