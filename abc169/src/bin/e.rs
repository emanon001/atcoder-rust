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
        abv: [(usize, usize); n]
    };
    if n % 2 == 0 {
        let mut v1 = abv.clone();
        v1.sort();
        let mut v2 = abv.clone();
        v2.sort_by_key(|x| x.1);
        let a = v2[n / 2 - 1].1 - v1[n / 2 - 1].0 + 1;
        let b = v2[n / 2].1 - v1[n / 2].0 + 1;
        let res = b + a - 1;
        println!("{}", res);
    } else {
        let mut v1 = abv.clone();
        v1.sort();
        let mut v2 = abv.clone();
        v2.sort_by_key(|x| x.1);
        let res = v2[n / 2].1 - v1[n / 2].0 + 1;
        println!("{}", res);
    }
    // let mut set = BTreeSet::new();
    // for a in 1..=2 {
    //     for b in 1..=2 {
    //         for c in 2..=5 {
    //             for d in 3..=4 {
    //                 let mut v = vec![a, b, c, d];
    //                 v.sort();
    //                 set.insert((v[1], v[2]));
    //             }
    //         }
    //     }
    // }
    // for (a, b) in set {
    //     println!("{}, {}", a, b);
    // }
}
