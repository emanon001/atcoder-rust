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
        n: usize, k: usize
    };

    let max = (n - 2 + 1) * (n - 2) / 2;
    if k > max {
        println!("-1");
        return;
    }

    let c = n - 1 + (max - k);
    println!("{}", c);
    // star
    for v in 2..=n {
        println!("{} {}", 1, v);
    }
    for v in (2..=n).combinations(2).take(max - k) {
        println!("{} {}", v[0], v[1]);
    }
}
