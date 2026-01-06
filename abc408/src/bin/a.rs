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
        N: usize, S: usize,
        T: [usize; N]
    };

    let is_ok = vec![0]
        .into_iter()
        .chain(T.into_iter())
        .tuple_windows()
        .all(|(a, b)| b - a <= S);
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}
