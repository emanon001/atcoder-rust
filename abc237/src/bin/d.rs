#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, s: &[char], res: &mut VecDeque<usize>) {
    if i == s.len() {
        res.push_front(i);
        return;
    }
    dfs(i + 1, s, res);
    if s[i] == 'L' {
        res.push_back(i);
    } else {
        res.push_front(i);
    }
}

fn solve() {
    input! {
        _n: usize,
        s: Chars
    };

    let mut res = VecDeque::new();
    dfs(0, &s, &mut res);
    println!("{}", res.iter().join(" "));
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
