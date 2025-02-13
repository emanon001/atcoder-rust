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
        S: Chars,
        Q: usize,
        K: [Usize1; Q],
    };

    let mut counts = vec![];
    let mut count = S.len();
    while count <= 10.pow(18) {
        counts.push(count);
        count *= 2;
    }
    let mut ans = vec![];
    for k in K {
        ans.push(dfs(&S, k, &counts));
    }
    println!("{}", ans.iter().join(" "));
}

fn dfs(s: &[char], pos: usize, counts: &[usize]) -> char {
    if pos < s.len() {
        return s[pos];
    }
    let mut _mod = 0;
    for &c in counts {
        if c > pos {
            break;
        }
        _mod = c;
    }
    let pos = pos % _mod;
    let ch = dfs(s, pos, counts);
    if ch.is_lowercase() {
        ch.to_ascii_uppercase()
    } else {
        ch.to_ascii_lowercase()
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
