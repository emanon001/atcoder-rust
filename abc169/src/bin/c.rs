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
        a: usize,
        b: Chars
    };

    let b: usize = b
        .into_iter()
        .filter(|&ch| ch != '.')
        .join("")
        .parse()
        .unwrap();
    let res = a * b / 100;
    println!("{}", res);
}
