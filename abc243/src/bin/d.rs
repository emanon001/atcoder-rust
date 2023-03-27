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
        _: usize, x: u64,
        s: Chars
    };

    let mut route = VecDeque::new();
    let mut x = x;
    while x > 1 {
        if x % 2 == 0 {
            route.push_front('L');
        } else {
            route.push_front('R');
        }
        x /= 2;
    }
    for ch in s {
        match ch {
            'U' => {
                route.pop_back();
            }
            'L' => {
                route.push_back('L');
            }
            'R' => {
                route.push_back('R');
            }
            _ => unreachable!(),
        }
    }
    let mut res = 1_u64;
    for c in route {
        if c == 'L' {
            res *= 2;
        } else {
            res = res * 2 + 1;
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
