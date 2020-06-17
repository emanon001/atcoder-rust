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
        mut s: Chars
    };

    s.push('R');
    let mut res = vec![0; s.len()];
    let mut is_right = true;
    let mut l = 0;
    for i in 1..s.len() {
        match s[i] {
            'R' => {
                if !is_right {
                    let len = i - l;
                    let c = len / 2;
                    res[l - 1] += c;
                    res[l] += len - c;
                    is_right = true;
                    l = i;
                }
            }
            'L' => {
                if is_right {
                    let len = i - l;
                    let c = len / 2;
                    res[i] += c;
                    res[i - 1] += len - c;
                    is_right = false;
                    l = i;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{}", res.into_iter().take(s.len() - 1).join(" "));
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
