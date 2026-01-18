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
        D: [usize; N - 1],
    };

    let mut ans = vec![];
    for i in 0..N - 1 {
        let mut v = vec![];
        let mut d = 0;
        for j in i..N - 1 {
            d += D[j];
            v.push(d);
        }
        ans.push(v.into_iter().join(" "));
    }
    println!("{}", ans.into_iter().join("\n"));
}
