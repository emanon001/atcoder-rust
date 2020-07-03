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
        n: usize, k: usize,
        r: usize, s: usize, p: usize,
        t: Chars
    };

    let mut res = 0_usize;
    for i in 0..k {
        let mut prev = 'x';
        let mut i = i;
        let mut sum = 0;
        while i < n {
            let ch = t[i];
            match ch {
                'r' => {
                    if prev != 'p' {
                        sum += p;
                        prev = 'p';
                    } else {
                        prev = 'x';
                    }
                }
                's' => {
                    if prev != 'r' {
                        sum += r;
                        prev = 'r';
                    } else {
                        prev = 'x';
                    }
                }
                'p' => {
                    if prev != 's' {
                        sum += s;
                        prev = 's';
                    } else {
                        prev = 'x';
                    }
                }
                _ => unreachable!(),
            }
            i += k;
        }
        res += sum;
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
