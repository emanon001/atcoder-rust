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
        mut rv: [i64; n]
    };

    rv.sort_by_key(|r| -r);
    let mut res = 0_f64;
    for i in 0..n {
        if i % 2 == 1 {
            continue;
        }
        let r = rv[i];
        let outer_area = std::f64::consts::PI * (r * r) as f64;
        res += outer_area;
        if i + 1 < n {
            let ir = rv[i + 1];
            let inner_area = std::f64::consts::PI * (ir * ir) as f64;
            res -= inner_area;
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
