#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn bit_dp<C>(
    state: usize,
    dp: &mut [Option<i64>],
    candidates: &[C],
    fin: usize,
    inf: i64,
    f: fn(usize, &C) -> (usize, i64),
) -> i64 {
    if let Some(res) = dp[state] {
        return res;
    }
    if state == fin {
        let res = 0;
        dp[state] = Some(res);
        return res;
    }
    let mut res = inf;
    for c in candidates {
        let (new_state, cost) = f(state, c);
        if new_state != state {
            let cost = bit_dp(new_state, dp, candidates, fin, inf, f) + cost;
            res = res.min(cost);
        }
    }
    dp[state] = Some(res);
    res
}

fn solve() {
    input! {
        n: usize, m: usize,
        coupon_v: [[usize; n]; m]
    };

    let inf = 1 << 30;
    let mut dp = vec![None; 1 << n];
    let res = bit_dp(0, &mut dp, &coupon_v, (1 << n) - 1, inf, |state, c| {
        let mut state = state;
        for (i, x) in c.iter().enumerate() {
            state |= x << i;
        }
        (state, 1)
    });
    let res = if res == inf { -1 } else { res };
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
