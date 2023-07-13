#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize, m: usize,
        av: [i64; n]
    };

    let mut cusum = vec![0_i64; n + 1];
    for i in 0..n {
        cusum[i + 1] = cusum[i] + av[i];
    }
    let mut cur = av
        .iter()
        .take(m)
        .enumerate()
        .fold(0_i64, |acc, (i, a)| acc + (i + 1) as i64 * a);
    let mut res = cur;
    for i in 0..n {
        // eprintln!("{}", cur);
        if i + m >= n {
            break;
        }
        let sub = cusum[i + m] - cusum[i];
        let add = m as i64 * av[i + m];
        cur = cur - sub + add;
        // eprintln!("cur - {} + {} = {}", sub, add, cur);
        chmax!(res, cur);
    }
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
