#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn counts(s: Vec<char>) -> Vec<(char, usize)> {
    let mut res = Vec::new();
    if s.is_empty() {
        return res;
    }
    let mut cur_ch = s[0];
    let mut count = 1;
    for &ch in &s[1..] {
        if ch == cur_ch {
            count += 1;
        } else {
            res.push((cur_ch, count));
            count = 1;
        }
        cur_ch = ch;
    }
    res.push((cur_ch, count));
    res
}

fn solve() {
    input! {
        s: Chars,
        t: Chars,
    };

    let s_counts = counts(s);
    let t_counts = counts(t);
    if s_counts.len() != t_counts.len() {
        println!("No");
        return;
    }
    let is_ok = s_counts
        .into_iter()
        .zip(t_counts)
        .all(|(s, t)| s.0 == t.0 && (s.1 == t.1 || s.1 >= 2 && s.1 <= t.1));
    let res = if is_ok { "Yes" } else { "No" };
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
