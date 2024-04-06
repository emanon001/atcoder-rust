#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn divisors(n: usize) -> usize {
    let mut res = 0;
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            res += 1;
            let y = n / x;
            if y != x {
                res += 1;
            }
        }
        x += 1;
    }
    res
}

#[allow(non_snake_case)]
fn solve() {
    input! {
        N: usize,
    };

    let mut divisors_counts = vec![0; N + 1];
    for x in 1..=N {
        divisors_counts[x] = divisors(x);
    }

    let mut ans = 0;
    for ab in 1..=N - 1 {
        let ab_c = divisors_counts[ab];
        let cd = N - ab;
        let cd_c = divisors_counts[cd];
        ans += ab_c * cd_c;
    }
    println!("{}", ans);
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
