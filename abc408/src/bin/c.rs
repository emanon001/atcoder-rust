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
        LR: [(Usize1, Usize1); M],
    };

    let mut imos = vec![0; N + 1];
    for (l, r) in LR {
        imos[l] += 1;
        imos[r + 1] -= 1;
    }
    for i in 0..N {
        imos[i + 1] += imos[i];
    }
    let ans = imos.iter().dropping_back(1).min().unwrap();
    println!("{}", ans);
}
