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

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        av: [u64; n]
    };

    let prime_factors = av.iter().copied().map(prime_factor).collect::<Vec<_>>();
    let mut min2 = 1000_u64;
    let mut min3 = 1000_u64;
    for pf in &prime_factors {
        chmin!(min2, *pf.get(&2).unwrap_or(&0));
        chmin!(min3, *pf.get(&3).unwrap_or(&0));
    }
    let new_av = av
        .into_iter()
        .zip(prime_factors)
        .map(|(a, pf)| {
            let c2 = *pf.get(&2).unwrap_or(&0) - min2;
            let c3 = *pf.get(&3).unwrap_or(&0) - min3;
            let a = a / 2_u64.pow(c2 as u32) / 3_u64.pow(c3 as u32);
            (a, c2 + c3)
        })
        .collect::<Vec<_>>();
    if new_av.iter().map(|it| it.0).unique().count() != 1 {
        println!("-1");
        return;
    }
    let res: u64 = new_av.into_iter().map(|it| it.1).sum();
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
