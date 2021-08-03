#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn divisors(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            res.push(x);
            let y = n / x;
            if y != x {
                res.push(y);
            }
        }
        x += 1;
    }
    res
}

fn solve() {
    input! {
        k: u64
    };

    let mut divs = divisors(k);
    divs.sort();

    let mut res = 0;
    for i in 0..divs.len() {
        let a = divs[i];
        for j in i..divs.len() {
            let b = divs[j];
            if (k / a) % b == 0 {
                let c = k / a / b;
                if c >= b {
                    res += 1;
                }
            }
        }
    }
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
