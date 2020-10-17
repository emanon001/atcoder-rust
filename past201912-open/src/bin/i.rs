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
        let (new_state, c) = f(state, c);
        if new_state != state {
            let cost = bit_dp(new_state, dp, candidates, fin, inf, f) + c;
            res = res.min(cost);
        }
    }
    dp[state] = Some(res);
    res
}

fn main() {
    input! {
        n: usize, m: usize,
        scv: [(Chars, i64); m]
    };

    let fin = (1 << n) - 1;
    let inf = 1_i64 << 60;
    let mut dp = vec![None; fin + 1];
    let cost = bit_dp(0, &mut dp, &scv, fin, inf, |state, (s, c)| {
        let mut new_state = state;
        for (i, ch) in s.iter().enumerate() {
            new_state |= if *ch == 'Y' { 1 << i } else { 0 };
        }
        (new_state, *c)
    });
    let res = if cost == inf { -1 } else { cost };
    println!("{}", res);
}
