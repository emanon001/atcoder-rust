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
        N: usize, M: usize,
        A: [usize; N]
    };

    let is_ok = A.into_iter().sum::<usize>() <= M;
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}
