#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [u64; N * 3]
    };

    let idx = (0..N * 3).collect_vec();
    let mut memo = HashMap::new();
    for comb_idx in idx.iter().combinations(3) {
        let sum = comb_idx.iter().map(|&i| A[*i]).sum::<u64>();
        let idx = idx
            .iter()
            .filter(|&i| !comb_idx.contains(&i))
            .copied()
            .collect_vec();
        for comb_idx2 in idx.iter().combinations(3) {
            let sum2 = comb_idx2.iter().map(|&i| A[*i]).sum::<u64>();
            let min = sum.min(sum2);
            let max = sum.max(sum2);
            let key = comb_idx.iter().chain(comb_idx2.iter()).sorted().join(",");
            memo.entry(key).or_insert(Vec::new()).push((min, max));
        }
    }
    let ans = dfs(u64::max_value(), 0, &idx, &A, &memo);
    println!("{}", ans);
}

fn dfs(
    min: u64,
    max: u64,
    idx: &[usize],
    a: &[u64],
    memo: &HashMap<String, Vec<(u64, u64)>>,
) -> u64 {
    if idx.len() == 6 {
        let mut res = u64::max_value();
        for &(min2, max2) in memo.get(&idx.iter().join(",")).unwrap() {
            let mut min = min;
            let mut max = max;
            chmin!(min, min2);
            chmax!(max, max2);
            chmin!(res, max - min);
        }
        return res;
    }

    let mut res = u64::max_value();
    for comb_idx in idx.iter().combinations(3) {
        let mut min = min;
        let mut max = max;
        let sum: u64 = comb_idx.iter().map(|&i| a[*i]).sum::<u64>();
        chmin!(min, sum);
        chmax!(max, sum);
        let rest_idx = idx
            .iter()
            .filter(|&i| !comb_idx.contains(&i))
            .copied()
            .collect::<Vec<_>>();
        chmin!(res, dfs(min, max, &rest_idx, a, memo));
    }
    res
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
