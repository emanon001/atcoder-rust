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

fn main() {
    input! {
        n: u64
    };

    let pf = prime_factor(n);
    let mut res = 0;
    for (_, c) in pf {
        let mut cur = c;
        let mut m = 1;
        while cur >= m {
            cur -= m;
            res += 1;
            m += 1;
        }
    }
    println!("{}", res);
}
