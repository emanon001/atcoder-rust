#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: i64,
    };

    let mut ans = vec![];
    for (i, v) in balanced_ternary(N).into_iter().enumerate() {
        match v {
            1 => ans.push(3_i64.pow(i as u32)),
            -1 => ans.push(-(3_i64.pow(i as u32))),
            _ => { /* do nothing */ }
        }
    }
    println!("{}", ans.len());
    println!("{}", ans.iter().join(" "));
}

fn balanced_ternary(n: i64) -> Vec<i64> {
    let mut res = vec![];
    let mut n = n;
    while n > 0 {
        match n % 3 {
            0 => res.push(0),
            1 => {
                res.push(1);
                n -= 1;
            }
            2 => {
                res.push(-1);
                n += 1;
            }
            _ => unreachable!(),
        };
        n /= 3;
    }
    res
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
