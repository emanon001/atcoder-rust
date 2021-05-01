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
        s: Chars
    };

    let mut reversed = false;
    let mut t = VecDeque::new();
    for ch in s {
        if ch == 'R' {
            reversed = !reversed;
        } else {
            if reversed {
                t.push_front(ch);
            } else {
                t.push_back(ch);
            }
        }
    }
    if reversed {
        t = t.into_iter().rev().collect::<VecDeque<char>>();
    }

    let mut stack = Vec::new();
    for ch in t {
        if let Some(p) = stack.pop() {
            if p != ch {
                stack.push(p);
                stack.push(ch);
            }
        } else {
            stack.push(ch);
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
