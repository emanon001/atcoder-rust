#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn is_k_palindrome(s: &[char], k: usize) -> bool {
    for i in 0..s.len() {
        if i + k > s.len() {
            break;
        }
        let mut ok = true;
        for j in 0..k / 2 {
            if s[i + j] != s[i + k - 1 - j] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }
    false
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, K: usize,
        S: Chars,
    };

    let mut ans = vec![];
    for perm in (0..N).permutations(N) {
        let mut s = vec![];
        for i in perm {
            s.push(S[i]);
        }
        if !is_k_palindrome(&s, K) {
            ans.push(s);
        }
    }
    let ans = ans.iter().unique().count();
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
