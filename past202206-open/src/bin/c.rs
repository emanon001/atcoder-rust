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
        N: u64, M: u64
    };

    let mut ans = vec![];
    for k in 1..=M {
        ans.push(
            if N.checked_pow(k as u32)
                .map(|x| x <= 1000000000)
                .unwrap_or(false)
            {
                'o'
            } else {
                'x'
            },
        );
    }
    println!("{}", ans.into_iter().join(""));
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
