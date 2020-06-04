#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn add(a: u128, c: &mut [usize], digit: usize) {
    for i in 0..digit {
        if (a >> i) & 1 == 1 {
            c[i] += 1;
        }
    }
}

fn subtract(a: u128, c: &mut [usize], digit: usize) {
    for i in 0..digit {
        if (a >> i) & 1 == 1 {
            c[i] -= 1;
        }
    }
}

fn solve() {
    input! {
        n: usize,
        av: [u128; n]
    };

    let digit = 20;
    let mut res = 0;
    let mut l = 0;
    let mut r = 0;
    let mut c = vec![0; digit];
    while l <= r && r < n {
        let a = av[r];
        add(a, &mut c, digit);
        let is_ok = c.iter().all(|&x| x <= 1);
        if is_ok {
            res += r - l + 1;
            r += 1;
        } else {
            subtract(av[l], &mut c, digit);
            subtract(av[r], &mut c, digit);
            l += 1;
        }
        // println!("({}, {}), {:?}", l, r, c);
    }

    println!("{}", res);
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024)
        .spawn(|| {
            solve();
        })
        .unwrap()
        .join()
        .unwrap();
}
