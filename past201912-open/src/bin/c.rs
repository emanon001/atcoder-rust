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
        mut v: [isize; 6]
    };

    v.sort_by_key(|x| -x);
    println!("{}", v[3 - 1]);
}
