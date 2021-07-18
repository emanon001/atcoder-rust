#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
    };

    let mut res = Vec::new();
    for bits in 0..1 << n {
        let mut stack = Vec::new();
        for i in 0..n {
            let ch = if (bits >> i) & 1 == 0 {
                '('
            } else {
                ')'
            };
            stack.push(ch);
        }
        let mut balance = 0;
        for i in 0..n {
            if stack[i] == '(' {
                balance += 1;
            } else {
                balance -= 1;
            }
            if balance < 0 {
                break;
            }
        }
        if balance == 0 {
            res.push(stack.iter().join(""));
        }
    }
    res.sort();
    for s in res {
        println!("{}", s);
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
