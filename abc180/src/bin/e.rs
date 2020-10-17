#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(state: usize, dp: &mut [Vec<Option<i64>>], u: usize, xyzv: &[(i64, i64, i64)]) -> i64 {
    if let Some(res) = dp[state][u] {
        return res;
    }
    if state == (1 << xyzv.len()) - 1 && u == 0 {
        dp[state][u] = Some(0);
        return 0;
    }
    let (cx, cy, cz) = xyzv[u];
    let mut res = 1_i64 << 60;
    for v in 0..xyzv.len() {
        let (x, y, z) = xyzv[v];
        let new_state = state | (1 << v);
        if new_state != state {
            let cost =
                ((x - cx).abs() + (y - cy).abs() + 0.max(cz - z)) + f(new_state, dp, v, xyzv);
            res = res.min(cost);
        }
    }
    dp[state][u] = Some(res);
    res
}

fn solve() {
    input! {
        n: usize,
        xyzv: [(i64, i64, i64); n]
    };

    let mut dp = vec![vec![None; n]; 1 << n];
    let res = f(0, &mut dp, 0, &xyzv);
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
