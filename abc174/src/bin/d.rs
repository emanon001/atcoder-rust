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
        cv: Chars
    };

    let mut res = 0;
    let mut l = 0;
    let mut r = n - 1;
    while l < r {
        if cv[l] == 'W' {
            if cv[r] == 'R' {
                res += 1;
                l += 1;
                r -= 1;
            } else {
                r -= 1;
            }
        } else {
            l += 1;
        }
    }
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
