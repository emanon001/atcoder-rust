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
        N: usize, T: Chars,
        SV: [Chars; N],
    };

    let mut ans = vec![];
    for (i, s) in SV.into_iter().enumerate() {
        let i = i + 1;
        if s.len() == T.len() {
            let mut eq_count = 0;
            for j in 0..s.len() {
                if s[j] == T[j] {
                    eq_count += 1;
                }
            }
            if eq_count >= T.len() - 1 {
                ans.push(i);
            }
        } else if s.len() == T.len() + 1 {
            let mut j = 0;
            while j < T.len() {
                if s[j] != T[j] {
                    break;
                }
                j += 1;
            }
            if j == T.len() || &s[j + 1..] == &T[j..] {
                ans.push(i);
            }
        } else if s.len() == T.len() - 1 {
            let mut j = 0;
            while j < s.len() {
                if s[j] != T[j] {
                    break;
                }
                j += 1;
            }
            if j == s.len() || &s[j..] == &T[j + 1..] {
                ans.push(i);
            }
        }
    }
    println!("{}", ans.len());
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
