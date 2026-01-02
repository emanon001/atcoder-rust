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
        T: Chars,
        U: Chars,
    };

    let is_ok = (0..T.len()).any(|i| {
        if i + U.len() > T.len() {
            return false;
        }
        (0..U.len()).all(|j| T[i + j] == '?' || T[i + j] == U[j])
    });
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}
