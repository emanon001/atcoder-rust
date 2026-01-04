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
        X: i64, Y: i64,
    };

    let mut count = 0;
    for a in 1..=6 {
        for b in 1..=6 {
            if a + b >= X || (a - b).abs() >= Y {
                count += 1;
            }
        }
    }
    let ans = count as f64 / 36.0;
    println!("{}", ans);
}
