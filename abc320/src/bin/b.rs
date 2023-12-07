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

fn solve() {
    input_interactive! {
        s: Chars
    };

    let mut ans = 0;
    for i in 0..s.len() {
        for j in i + 1..=s.len() {
            let s = &s[i..j];
            if s.iter().join("") == s.iter().rev().collect::<String>() {
                chmax!(ans, s.len());
            }
        }
    }
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
