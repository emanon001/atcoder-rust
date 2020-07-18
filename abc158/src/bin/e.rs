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
        n: usize, p: usize,
        s: Chars
    };

    let s = s
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    if p == 2 || p == 5 {
        let mut res = 0_usize;
        for i in 0..n {
            if s[i] % p == 0 {
                res += i + 1;
            }
        }
        println!("{}", res);
        return;
    }

    let mut cusum = Vec::new();
    cusum.push(0);
    let mut ten_factor = 1_usize;
    for i in (0..n).rev() {
        let x = s[i];
        let sum = (cusum.last().unwrap() + (ten_factor * x) % p) % p;
        cusum.push(sum);
        ten_factor = (ten_factor * 10) % p;
    }
    let mut map = HashMap::new();
    for s in cusum {
        *map.entry(s).or_insert(0_usize) += 1;
    }
    let mut res = 0_usize;
    for (_, c) in map {
        if c < 2 {
            continue;
        }
        res += c * (c - 1) / 2;
    }
    println!("{}", res);
}
