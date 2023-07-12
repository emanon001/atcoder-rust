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
        _n: usize,
        s: Chars
    };

    let mut left_parent_count = 0;
    let mut stack = Vec::new();
    for ch in s {
        match ch {
            '(' => {
                left_parent_count += 1;
                stack.push(ch);
            }
            ')' => {
                if left_parent_count > 0 {
                    left_parent_count -= 1;
                    loop {
                        if stack.pop().unwrap() == '(' {
                            break;
                        }
                    }
                } else {
                    stack.push(')');
                }
            }
            ch => {
                stack.push(ch);
            }
        }
    }
    println!("{}", stack.iter().join(""));
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
