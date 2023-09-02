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
        h: usize, w: usize,
        s: [Chars; h],
        t: [Chars; h],
    };

    let mut s2 = vec![Vec::new(); w];
    let mut t2 = vec![Vec::new(); w];
    for i in 0..w {
        for j in 0..h {
            s2[i].push(s[j][i]);
            t2[i].push(t[j][i]);
        }
    }
    s2.sort();
    t2.sort();
    let res = if s2 == t2 { "Yes" } else { "No" };
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
