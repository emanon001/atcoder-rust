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
        lrv: [(u64, u64); n]
    };

    let mut res = 0_f64;
    for i in 0..n {
        let (l1, r1) = lrv[i];
        for j in i + 1..n {
            let (l2, r2) = lrv[j];
            let all_c = (r1 - l1 + 1) * (r2 - l2 + 1);
            let mut c = 0_u64;
            for v1 in l1..=r1 {
                for v2 in l2..=r2 {
                    if v1 > v2 {
                        c += 1;
                    }
                }
            }
            res += c as f64 / all_c as f64;
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
