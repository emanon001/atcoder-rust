#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn prime_factor(n: u64) -> std::collections::HashMap<u64, u64> {
    if n < 2 {
        return std::collections::HashMap::new();
    }
    let mut res = std::collections::HashMap::new();
    let mut n = n;
    let mut i = 2;
    while i * i <= n {
        while n % i == 0 {
            let c = res.entry(i).or_insert(0);
            *c += 1;
            n /= i;
        }
        i += 1;
    }
    if n != 1 {
        res.insert(n, 1);
    }
    res
}

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

fn solve() {
    input! {
        n: usize, m: usize,
        av: [u64; n]
    };

    let mut set = BTreeSet::new();
    for a in av {
        let pf = prime_factor(a);
        for (p, _) in pf {
            set.insert(p);
        }
    }
    let mut vec = vec![true; m + 1];
    for x in set {
        let mut a = 1;
        while x * a <= m as u64 {
            vec[(x * a) as usize] = false;
            a += 1;
        }
    }
    let mut res = Vec::new();
    res.push(1);
    for x in 2..=m {
        if vec[x] {
            res.push(x);
        }
    }
    println!("{}", res.len());
    for x in res {
        println!("{}", x);
    }
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
