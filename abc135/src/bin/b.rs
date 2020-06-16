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
        n: usize,
        pv: [usize; n]
    };

    let mut sorted = pv.clone();
    sorted.sort();
    let c = pv.into_iter().zip(sorted).filter(|&(a, b)| a != b).count();
    let res = if c == 0 || c == 2 { "YES" } else { "NO" };
    println!("{}", res);
}
