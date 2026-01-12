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
        T: Chars,
        A: Chars,
    };

    let is_ok = T
        .into_iter()
        .zip(A.into_iter())
        .any(|(t, a)| t == 'o' && a == 'o');
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}
