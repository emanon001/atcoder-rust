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
        av: [u128; n]
    };

    if av.contains(&0) {
        println!("0");
        std::process::exit(0);
    }

    let max = 1000000000000000000_u128;
    let mut cur = 1_u128;
    for a in av {
        cur *= a;
        if cur > max {
            println!("-1");
            std::process::exit(0);
        }
    }
    println!("{}", cur);
}
