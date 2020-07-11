#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let res = n % pop_count(n);
    f(res) + 1
}

fn pop_count(n: usize) -> usize {
    let mut res = 0;
    let mut n = n;
    while n > 0 {
        res += n % 2;
        n /= 2;
    }
    res
}

fn solve() {
    input! {
        n: usize,
        x: Chars
    };

    let x = x
        .into_iter()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();
    let mut pc = 0;
    for n in 0..n {
        if x[n] == 1 {
            pc += 1;
        }
    }
    let sum_mod_plus = x.iter().copied().fold(0, |acc, x| (acc * 2 + x) % (pc + 1));
    let sum_mod_minus = x.iter().copied().fold(0, |acc, x| {
        if pc <= 1 {
            return 0;
        }
        (acc * 2 + x) % (pc - 1)
    });
    let digits_val_mod_plus = (0..n)
        .into_iter()
        .scan(1, |acc, _| {
            let res = *acc;
            *acc = (*acc * 2) % (pc + 1);
            Some(res)
        })
        .collect::<Vec<_>>();
    let digits_val_mod_minus = (0..n)
        .into_iter()
        .scan(1, |acc, _| {
            if pc <= 1 {
                return Some(0);
            }
            let res = *acc;
            *acc = (*acc * 2) % (pc - 1);
            Some(res)
        })
        .collect::<Vec<_>>();
    for i in 0..n {
        if x[i] == 0 {
            let res = (sum_mod_plus + digits_val_mod_plus[n - i - 1]) % (pc + 1);
            let res = 1 + f(res);
            println!("{}", res);
        } else {
            if pc == 1 {
                println!("0");
            } else if digits_val_mod_minus[n - i - 1] > sum_mod_minus {
                let res = (pc - 1) + sum_mod_minus - digits_val_mod_minus[n - i - 1];
                let res = 1 + f(res);
                println!("{}", res);
            } else {
                let res = sum_mod_minus - digits_val_mod_minus[n - i - 1];
                let res = 1 + f(res);
                println!("{}", res);
            }
        };
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
