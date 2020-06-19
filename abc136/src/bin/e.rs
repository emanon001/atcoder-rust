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
        n: usize, k: u64,
        av: [u64; n]
    };

    let sum = av.iter().sum::<u64>();
    let mut divisors = divisors(sum);
    divisors.sort_by_key(|&x| -(x as i64));
    for d in divisors {
        let mut dv = av.iter().map(|x| x % d).collect::<Vec<_>>();
        dv.sort();
        let mcusum = dv
            .iter()
            .scan(0, |acc, &x| {
                *acc += x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        let pcusum = dv
            .iter()
            .scan(0, |acc, &x| {
                *acc += d - x;
                Some(*acc)
            })
            .collect::<Vec<_>>();
        for i in 0..n {
            let msum = mcusum[i];
            let psum = pcusum[n - 1] - pcusum[i];
            let c = std::cmp::max(msum, psum);
            let diff = (msum as i64 - psum as i64).abs() as u64;
            if diff % d == 0 && c <= k {
                println!("{}", d);
                return;
            }
        }
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
