#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: i64, m: i64
    };

    let x = m - 2 * n;
    if x < 0 {
        println!("-1 -1 -1");
        return;
    }

    for b in 0..=x {
        let c = (x - b) / 2;
        if c < 0 {
            continue;
        }
        if b + 2 * c != x {
            continue;
        }
        let a = (m - (3 * b + 4 * c)) / 2;
        if a < 0 {
            continue;
        }
        if a + b + c != n || 2 * a + 3 * b + 4 * c != m {
            continue;
        }
        println!("{} {} {}", a, b, c);
        return;
    }
    println!("-1 -1 -1");
}
