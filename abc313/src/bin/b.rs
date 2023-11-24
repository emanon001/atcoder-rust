#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize, m: usize,
        ab: [(Usize1, Usize1); m]
    };

    let mut indegree = vec![0; n];
    for (_, b) in ab {
        indegree[b] += 1;
    }
    let v = indegree
        .into_iter()
        .enumerate()
        .filter(|(_, x)| *x == 0)
        .map(|(i, _)| i + 1)
        .collect::<Vec<_>>();
    let ans = if v.len() == 1 {
        v[0].to_string()
    } else {
        "-1".to_owned()
    };
    println!("{ans}");
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
