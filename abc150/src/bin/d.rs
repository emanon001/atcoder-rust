#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn two_count(n: u64) -> u32 {
    let mut n = n;
    let mut res = 0;
    while n % 2 == 0 {
        res += 1;
        n /= 2;
    }
    res
}

fn solve() {
    input! {
        n: usize, m: u64,
        av: [u64; n]
    };

    let two_counts = av.iter().map(|&a| two_count(a)).collect::<Vec<_>>();
    let is_ok = two_counts.iter().collect::<HashSet<_>>().len() == 1;
    if !is_ok {
        println!("0");
        return;
    }
    let two_p = two_counts[0];
    let odd_lcm = av
        .iter()
        .map(|&a| {
            let m = 2.pow(two_p);
            a / m
        })
        .fold(1, |acc, x| acc.lcm(&x));
    let first_gcd = 2_u64.pow(two_p - 1) * odd_lcm;
    let res = if first_gcd > m {
        0
    } else {
        1 + (m - first_gcd) / (first_gcd * 2)
    };
    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
