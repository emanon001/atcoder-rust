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
        X: u64, Y: u64
    };

    let xc = divisors(X).len();
    let yc = divisors(Y).len();
    let ans = match xc.cmp(&yc) {
        std::cmp::Ordering::Greater => "X",
        std::cmp::Ordering::Less => "Y",
        std::cmp::Ordering::Equal => "Z",
    };
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
