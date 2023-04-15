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
        q: usize,
    };

    let mut v = VecDeque::new();
    for _ in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    x: i64, c: i64
                };
                v.push_back((x, c));
            }
            2 => {
                input! {
                    c: i64
                };
                let mut sum = 0_i64;
                let mut rest_count = c;
                while rest_count > 0 {
                    let (x, c2) = v.pop_front().unwrap();
                    let choice_c = (c2).min(rest_count);
                    rest_count -= choice_c;
                    sum += x * choice_c;
                    if choice_c < c2 {
                        v.push_front((x, c2 - choice_c));
                    }
                }
                println!("{}", sum);
            }
            _ => unreachable!(),
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
