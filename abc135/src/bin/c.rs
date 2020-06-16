#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn main() {
    input! {
        n: usize,
        mut av: [usize; n + 1],
        mut bv: [usize; n],
    };

    let mut res = 0_u64;
    for i in 0..n {
        let x = std::cmp::min(bv[i], av[i]);
        res += x as u64;
        av[i] -= x;
        bv[i] -= x;
        let x = std::cmp::min(bv[i], av[i + 1]);
        res += x as u64;
        av[i + 1] -= x;
        bv[i] -= x;
    }
    println!("{}", res);
}
