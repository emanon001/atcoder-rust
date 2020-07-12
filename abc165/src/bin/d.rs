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
        a: u64, b: u64, n: u64
    };

    if n < b {
        println!("{}", (a * n / b) - (a * (n / b)));
    } else {
        println!("{}", (a * (b - 1) / b) - (a * ((b - 1) / b)));
    }
}
