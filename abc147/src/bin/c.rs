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
        xyvv: [[(Usize1, usize)]]
    };

    let n = xyvv.len();
    let mut res = 0;
    for bits in 0..(1 << n) {
        let mut honest_set = HashSet::new();
        let mut unkindness_set = HashSet::new();
        for i in 0..n {
            let is_honest = (bits >> i) & 1 == 1;
            if is_honest {
                honest_set.insert(i);
            } else {
                unkindness_set.insert(i);
            };
            for &(x, y) in &xyvv[i] {
                if is_honest {
                    if y == 0 {
                        unkindness_set.insert(x);
                    } else {
                        honest_set.insert(x);
                    }
                }
            }
        }
        let is_ok = honest_set.intersection(&unkindness_set).count() == 0;
        if is_ok && honest_set.len() > res {
            res = honest_set.len();
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
