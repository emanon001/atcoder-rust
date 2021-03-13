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
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        a: usize, b: usize, w: usize
    };

    let w = w * 1000;
    let inf = 1 << 30;
    let mut min_dp = vec![inf; w + 1];
    min_dp[0] = 0;
    let mut max_dp = vec![0; w + 1];
    max_dp[0] = 0;
    for i in 0..w {
        for j in a..=b {
            if i + j > w {
                break;
            }
            chmin!(min_dp[i + j], min_dp[i] + 1);
            chmax!(max_dp[i + j], max_dp[i] + 1);
        }
    }
    if min_dp[w] == inf || max_dp[w] == 0 {
        println!("UNSATISFIABLE");
        return;
    }
    println!("{} {}", min_dp[w], max_dp[w]);
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
