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
        P: Chars,
        L: usize,
    };

    let is_ok = P.len() >= L;
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}
