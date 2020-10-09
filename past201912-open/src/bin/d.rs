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
        av: [Usize1; n]
    };

    let mut counts = vec![0; n];
    for a in av {
        counts[a] += 1;
    }
    if counts.iter().collect::<HashSet<_>>().len() == 1 {
        println!("Correct");
    } else {
        let x = counts.iter().position(|&a| a == 0).unwrap() + 1;
        let y = counts.iter().position(|&a| a == 2).unwrap() + 1;
        println!("{} {}", y, x);
    }
}
