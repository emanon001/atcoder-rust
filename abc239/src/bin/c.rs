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
        x1: i64, y1: i64, x2: i64, y2: i64,
    };

    for ax in -5_i64..=5_i64 {
        for ay in -5_i64..=5_i64 {
            let x3 = x1 + ax;
            let y3 = y1 + ay;
            let d1 = (x1 - x3).pow(2) + (y1 - y3).pow(2);
            let d2 = (x2 - x3).pow(2) + (y2 - y3).pow(2);
            if d1 == 5 && d2 == 5 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
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
