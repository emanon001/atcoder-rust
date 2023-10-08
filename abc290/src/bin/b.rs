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
        n: usize, k: usize,
        s: Chars
    };

    let mut c = 0;
    let mut list = vec!['x'; n];
    for (i, ch) in s.into_iter().enumerate() {
        if ch == 'o' && c < k {
            list[i] = 'o';
            c += 1;
        }
    }
    let ans = list.into_iter().join("");
    println!("{}", ans);
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
