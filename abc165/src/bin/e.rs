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
        n: usize, m: usize
    };

    let mut res = Vec::new();
    if n % 2 == 0 {
        let mut i = 1;
        let mut changed = false;
        for _ in 0..m {
            let j = n - i + if changed { 1 } else { 0 };
            if !changed && n - j + i >= j - i {
                changed = true;
                i += 1;
            }
            res.push((i, j));
            i += 1;
        }
    } else {
        for i in 1..m + 1 {
            res.push((i, n - i));
        }
    }
    for (a, b) in res {
        println!("{} {}", a, b);
    }
}
