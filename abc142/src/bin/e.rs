#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn f(
    state: usize,
    dp: &mut [Option<usize>],
    av: &[usize],
    cv: &[usize],
    fin: usize,
    inf: usize,
) -> usize {
    if let Some(res) = dp[state] {
        return res;
    }
    if state == fin {
        let res = 0;
        dp[state] = Some(res);
        return res;
    }
    let mut res = inf;
    for i in 0..av.len() {
        let a = av[i];
        let c = cv[i];
        let new_state = state | c;
        if new_state != state {
            let cost = a + f(new_state, dp, av, cv, fin, inf);
            res = res.min(cost);
        }
    }
    dp[state] = Some(res);
    res
}

fn solve() {
    input! {
        n: usize, m: usize
    };

    let mut av = Vec::new();
    let mut cvv = Vec::new();
    for _ in 0..m {
        input! {
            a: usize, b: usize,
            cv: [Usize1; b]
        };
        av.push(a);
        cvv.push(cv);
    }
    let fin = (1 << n) - 1;
    let inf = 1 << 60;
    let mut dp = vec![None; fin + 1];
    let cv = cvv
        .into_iter()
        .map(|cv| cv.into_iter().fold(0, |acc, c| acc | (1 << c)))
        .collect::<Vec<_>>();
    let cost = f(0, &mut dp, &av, &cv, fin, inf);
    let res = if cost == inf { -1 } else { cost as isize };
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
