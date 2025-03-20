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
        S: Chars,
    };
    let mut ans = 0;
    let mut is_odd = true;
    for ch in S {
        match (is_odd, ch) {
            (false, 'i') => {
                ans += 1;
            }
            (false, 'o') => {
                is_odd = !is_odd;
            }
            (true, 'i') => {
                is_odd = !is_odd;
            }
            (true, 'o') => {
                ans += 1;
            }
            _ => unreachable!(),
        }
    }
    if !is_odd {
        ans += 1;
    }
    println!("{}", ans);
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
