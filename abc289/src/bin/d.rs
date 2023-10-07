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
        n: usize,
        av: [usize; n],
        m: usize,
        bv: [usize; m],
        x: usize,
    };

    let bv = bv.into_iter().collect::<HashSet<_>>();
    let mut dp = vec![None; x + 1];
    let is_ok = dfs(x, &av, &bv, &mut dp);
    let ans = if is_ok { "Yes" } else { "No" };
    println!("{}", ans);
}

fn dfs(floor: usize, av: &[usize], bv: &HashSet<usize>, dp: &mut Vec<Option<bool>>) -> bool {
    if floor == 0 {
        return true;
    }
    if bv.contains(&floor) {
        return false;
    }
    if let Some(ans) = dp[floor] {
        return ans;
    }
    let mut ans = false;
    for &a in av {
        if floor >= a {
            ans = ans || dfs(floor - a, av, bv, dp);
        }
    }
    dp[floor] = Some(ans);
    ans
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
