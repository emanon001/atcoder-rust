#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn divisors(n: usize, counts: &mut [usize]) {
    let mut x = 1;
    while x * x <= n {
        if n % x == 0 {
            counts[x] += 1;
            let y = n / x;
            if y != x {
                counts[y] += 1;
            }
        }
        x += 1;
    }
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, K: usize,
        A: [usize; N]
    };

    let mut counts = vec![0; 10.pow(6) + 1];
    for a in A {
        divisors(a, &mut counts);
    }
    for (i, &c) in counts.iter().enumerate().rev() {
        if c >= K {
            println!("{}", i);
            return;
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
