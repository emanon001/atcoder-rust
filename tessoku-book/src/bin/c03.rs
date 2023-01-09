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
        d: usize,
        av: [i64; d],
        q: usize,
        stv: [(Usize1, Usize1); q]
    };

    let cusum = av
        .into_iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect::<Vec<_>>();
    for (s, t) in stv {
        let res = if cusum[s] > cusum[t] {
            (s + 1).to_string()
        } else if cusum[t] > cusum[s] {
            (t + 1).to_string()
        } else {
            "Same".to_string()
        };
        println!("{}", res);
    }
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
