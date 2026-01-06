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
        S: Chars
    };

    let s = S
        .into_iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();
    let mut last_d = 0;
    let mut ans = 0;
    for d in s.into_iter().rev() {
        // push button A
        ans += 1;
        // push button B
        let c = if d < last_d {
            d + 10 - last_d
        } else {
            d - last_d
        };
        ans += c;
        last_d = (last_d + c) % 10;
    }
    println!("{}", ans);
}
