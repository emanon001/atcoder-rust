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
        a: isize, b: isize
    };

    let diff = (a - b).abs();
    if diff % 2 == 0 {
        let res = std::cmp::min(a, b) + diff / 2;
        println!("{}", res);
    } else {
        let res = "IMPOSSIBLE";
        println!("{}", res);
    };
}
