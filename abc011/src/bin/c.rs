#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, c: usize, ng_set: &HashSet<usize>, dp: &mut Vec<Option<usize>>) {
    if ng_set.contains(&i) {
        return;
    }
    if let Some(dc) = dp[i] {
        if c >= dc {
            return;
        }
    }
    dp[i] = Some(c);
    for m in (1..=3).rev() {
        if i >= m {
            dfs(i - m, c + 1, ng_set, dp);
        }
    }
}

fn solve() {
    input! {
        n: usize,
        ngv: [usize; 3]
    };

    let ng_set = ngv.iter().copied().collect::<HashSet<_>>();
    let mut dp = vec![None; n + 1];
    dfs(n, 0, &ng_set, &mut dp);
    let res = if dp[0].unwrap_or(1 << 30) <= 100 {
        "YES"
    } else {
        "NO"
    };
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
