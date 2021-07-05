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
        n: u64
    };

    if n == 1 {
        println!("2");
        return;
    }

    let divs = divisors(n);
    let mut res = 0_u64;
    if n % 2 == 0 {
        for d in divs {
            if d % 2 == 0 && (n / d) % 2 == 1 {
                res += 2;
            }
        }
    } else {
        for d in divs {
            if d % 2 == 1 && (n / d) % 2 == 1 {
                res += 2;
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
