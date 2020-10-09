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
        s: String
    };

    let res = if let Ok(x) = s.parse::<usize>() {
        (x * 2).to_string()
    } else {
        "error".into()
    };
    println!("{}", res);
}
