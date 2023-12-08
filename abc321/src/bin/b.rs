#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize, x: i64,
        a: [i64; n - 1]
    };

    let mut ans: Option<i64> = None;
    for s in 0..=100 {
        let a = a
            .clone()
            .into_iter()
            .chain(vec![s])
            .sorted()
            .collect::<Vec<_>>();
        let sum = a[1..n - 1].iter().sum::<i64>();
        if sum >= x {
            ans = match ans {
                Some(m) => Some(m.min(s)),
                None => Some(s),
            }
        }
    }
    println!("{}", ans.unwrap_or(-1));
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
