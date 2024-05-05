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
        T: Chars
    };

    let mut ans = vec![];
    let mut s_i = 0;
    for (i, ch) in T.into_iter().enumerate() {
        if s_i >= S.len() {
            continue;
        }
        if S[s_i] == ch {
            ans.push(i + 1);
            s_i += 1;
        }
    }
    println!("{}", ans.iter().join(" "));
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
