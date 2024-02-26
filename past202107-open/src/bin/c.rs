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
        S: String,
        L: u64, R: u64,
    };

    if S.starts_with('0') && S.len() > 1 {
        println!("No");
        return;
    }
    if S.len() > 9 {
        println!("No");
        return;
    }
    let s = S.parse::<u64>().unwrap();
    let ans = if s >= L && s <= R { "Yes" } else { "No" };
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
