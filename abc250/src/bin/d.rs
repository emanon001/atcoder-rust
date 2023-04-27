#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn primes(n: usize) -> Vec<usize> {
    if n < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut res = Vec::new();
    for i in 2..=n {
        if is_prime[i] {
            res.push(i);
            let mut j = 2 * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    res
}

pub fn bsearch<F>(ok: i64, ng: i64, pred: F) -> Option<i64>
where
    F: Fn(i64) -> bool,
{
    let orig_ok = ok;
    let mut ok = ok;
    let mut ng = ng;
    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;
        if pred(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    if ok == orig_ok {
        None
    } else {
        Some(ok)
    }
}

fn solve() {
    input! {
        n: i128
    };

    let primes = primes(10.pow(6));
    let mut res = 0_i64;
    for i in 0..primes.len() {
        let p = primes[i] as i128;
        let c = bsearch(i as i64, primes.len() as i64, |j| {
            let j = j as usize;
            let q = primes[j] as i128;
            p * q.pow(3) <= n
        })
        .map(|j| j - i as i64)
        .unwrap_or(0);
        res += c;
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
