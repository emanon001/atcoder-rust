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
        av: [i64; n]
    };

    let mut prev = av[0];
    for a in av.into_iter().skip(1) {
        let res = if a == prev {
            "stay".to_string()
        } else if a < prev {
            format!("down {}", prev - a)
        } else {
            format!("up {}", a - prev)
        };
        println!("{}", res);
        prev = a;
    }
}
