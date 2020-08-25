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
        a: i64
    };

    for k in 10_i64..=10000 {
        let mut n = 0;
        let mut m = 1;
        let mut rest = k;
        while rest > 0 {
            n += (rest % 10) * m;
            rest = rest / 10;
            if rest > 0 {
                m *= k;
            }
        }
        if n == a {
            println!("{}", k);
            return;
        }
    }
    println!("-1");
}
