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
        b: i64, c: i64
    };

    if b == 0 {
        let res = c / 2 + (c - 2) / 2 + 1;
        println!("{}", res);
        return;
    }

    let count1 = c / 2 + (c - 2) / 2 + 1; // B基準
    let count2 = (c - 1) / 2 + (c - 1) / 2 + 1; // -B基準
    // x座標上で左と右の点を基準とした整数の数
    let (left_b_count, right_b_count) = if b >= 0 {
        (count2, count1)
    } else {
        (count1, count2)
    };
    let mut res = left_b_count + right_b_count;
    let (left_b_r, right_b_l) = if b >= 0 {
        (-b + (c - 1) / 2, b - c / 2)
    } else {
        (b + (c - 2) / 2, -b - (c - 1) / 2)
    };
    if left_b_r >= right_b_l {
        res -= left_b_r - right_b_l + 1;
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
