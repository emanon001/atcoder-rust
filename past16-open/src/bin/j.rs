#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, K: usize,
        A: [u64; K]
    };

    let min_distance = A
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .reduce(|a, b| a.gcd(&b))
        .unwrap();
    let l = *A.iter().min().unwrap();
    let r = *A.iter().max().unwrap();
    let mut ans = BTreeSet::new();
    for d in divisors(min_distance) {
        if (r - l) / d < N as u64 {
            ans.insert(d);
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
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
