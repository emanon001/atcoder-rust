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
        _: usize,
        S: Chars,
        T: Chars,
    };

    let ans = if check(&S, &T) { "Yes" } else { "No" };
    println!("{}", ans);
}

fn check(s: &[char], t: &[char]) -> bool {
    if s.len() == t.len() {
        let mut diff = 0;
        for i in 0..s.len() {
            if s[i] != t[i] {
                diff += 1;
            }
        }
        diff <= 1
    } else if s.len() + 1 == t.len() || s.len() == t.len() + 1 {
        let (a, b) = if s.len() + 1 == t.len() {
            (s, t)
        } else {
            (t, s)
        };

        let mut i = 0;
        while i < a.len() {
            if a[i] != b[i] {
                break;
            }
            i += 1;
        }
        let mut diff = 0;
        let mut j = i + 1;
        while i < a.len() && j < b.len() {
            if a[i] != b[j] {
                diff += 1;
            }
            i += 1;
            j += 1;
        }
        diff == 0
    } else {
        false
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
