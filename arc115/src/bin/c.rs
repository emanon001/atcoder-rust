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
        n: usize
    };

    let mut res = vec![1; n];
    for i in 1..=n {
        let x = res[i - 1];
        let mut j= i * 2;
        while j <= n {
            res[j - 1] = if res[j - 1] == x { x + 1 } else { res[j - 1] };
            j += i;
        }
    }
    println!("{}", res.into_iter().join(" "));
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
