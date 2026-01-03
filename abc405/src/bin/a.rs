#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn main() {
    input_interactive! {
        R: i64, X: i64,
    };

    let is_rated = if X == 1 {
        (1600..=2999).contains(&R)
    } else {
        (1200..=2399).contains(&R)
    };
    let ans = if is_rated { "Yes" } else { "No" };
    println!("{}", ans);
}
