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
        N: usize,
        A: [usize; N],
        K: usize,
    };

    let ans = A.into_iter().filter(|&a| a >= K).count();
    println!("{}", ans);
}
