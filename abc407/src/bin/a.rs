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
        A: u32, B: u32,
    };

    let mut ans = A / B;
    if (A % B) * 2 > B {
        ans += 1;
    }
    println!("{}", ans);
}
