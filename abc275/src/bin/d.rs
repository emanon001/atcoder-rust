#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(k: u64, dp: &mut HashMap<u64, u64>) -> u64 {
    if k == 0 {
        return 1;
    }
    if let Some(res) = dp.get(&k) {
        return *res;
    }

    let res = dfs(k / 2, dp) + dfs(k / 3, dp);
    dp.insert(k, res);
    res
}

fn solve() {
    input! {
        n: u64
    };

    let mut dp = HashMap::new();
    let res = dfs(n, &mut dp);
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
