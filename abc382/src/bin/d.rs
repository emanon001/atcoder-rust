#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: isize, M: isize,
    };

    let mut dp = Vec::new();
    for a in 1..=10 {
        dfs(&mut vec![a], N, M, &mut dp);
    }
    println!("{}", dp.len());
    println!("{}", dp.join("\n"));
}

fn dfs(v: &mut Vec<isize>, n: isize, m: isize, dp: &mut Vec<String>) {
    if v.len() == n as usize {
        dp.push(v.iter().join(" "));
        return;
    }
    let prev = v[v.len() - 1];
    let s = prev + 10;
    if s > m {
        return;
    }
    if (m - s - (n - v.len() as isize - 1) * 10) < 0 {
        return;
    }
    let e = (s + 10).min(m);
    for a in s..=e {
        v.push(a);
        dfs(v, n, m, dp);
        v.pop();
    }
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
