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
        a: u128, b: u128
    };

    // answer1: 
    // let n = a.lcm(&b);
    // let res = if n > 10.pow(18) { "Large".to_string() } else { n.to_string() };

    let gcd = a.gcd(&b);
    let c = b / gcd;
    let res = if c > 10.pow(18) / a { "Large".to_string() } else { (a * c).to_string() };
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
