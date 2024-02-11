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
        N: u64,
    };

    let mut dp = HashMap::new();
    let ans = sum(N, &mut dp);
    println!("{}", ans);
}

fn sum(n: u64, dp: &mut HashMap<u64, u64>) -> u64 {
    if n < 2 {
        return 0;
    }
    if let Some(x) = dp.get(&n) {
        return *x;
    }
    let ans = sum(n / 2, dp) + sum((n + 1) / 2, dp) + n;
    dp.insert(n, ans);
    ans
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
