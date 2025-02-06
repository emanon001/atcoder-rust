#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
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

fn pow8(n: u64) -> Option<u64> {
    let mut m = n;
    for _ in 0..7 {
        m = m.checked_mul(n)?;
    }
    Some(m)
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: u64,
    };

    let primes = primes(10.pow(6));
    let mut ans = HashSet::new();
    for i in 0..primes.len() {
        let p = primes[i] as u64;
        match pow8(p) {
            Some(m) => {
                if m <= N {
                    ans.insert(m);
                }
            }
            None => {}
        }
        for j in i + 1..primes.len() {
            let q = primes[j] as u64;
            let m = p.pow(2).checked_mul(q.pow(2));
            match m {
                Some(m) => {
                    if m <= N {
                        ans.insert(p.pow(2) * q.pow(2));
                    }
                }
                None => break,
            };
        }
    }
    println!("{}", ans.len());
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
