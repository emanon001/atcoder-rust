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
        T: Chars,
    };

    let T = T
        .into_iter()
        .map(|ch| ch.to_lowercase().to_string())
        .join("")
        .chars()
        .collect::<Vec<_>>();

    let is_last_x = T[T.len() - 1] == 'x';
    let mut i = 0;
    for ch in S {
        if i < T.len() && ch == T[i] {
            i += 1;
        }
    }
    let ans = if i == 3 || (is_last_x && i >= 2) {
        "Yes"
    } else {
        "No"
    };
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
