#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

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

fn is_primely(n: u64) -> bool {
    if is_prime(n) {
        return true;
    }

    if divisors(n).len() <= 2 {
        return false;
    }

    let m = n % 10;
    let dsum = n
        .to_string()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u64)
        .sum::<u64>();
    (m % 2 != 0 && m != 5) && dsum % 3 != 0
}

fn solve() {
    input! {
        n: u64
    };

    let res = if is_primely(n) { "Prime" } else { "Not Prime" };
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
