#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn change(a: usize, b: usize, n: usize, s1: &mut[char], s2: &mut[char]) {
    match (a > n, b > n) {
        (false, false) => {
            let ch = s1[a - 1];
            s1[a - 1] = s1[b - 1];
            s1[b - 1] = ch;
        }
        (false, true) => {
            let ch = s1[a - 1];
            s1[a - 1] = s2[b - n - 1];
            s2[b - n - 1] = ch;
        }
        (true, true) => {
            let ch = s2[a - n - 1];
            s2[a - n - 1] = s2[b - n - 1];
            s2[b - n - 1] = ch;
        }
        (true, false) => {
            let ch = s2[a - n - 1];
            s2[a - n - 1] = s1[b - 1];
            s1[b - 1] = ch;
        }
    }
}

fn solve() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        queries: [(usize, usize, usize); q]
    };

    let mut s1 = s.iter().take(n).copied().collect::<Vec<char>>();
    let mut s2 = s.iter().skip(n).copied().collect::<Vec<char>>();
    let mut swapped = false;
    for (op, a, b) in queries {
        if op == 1 {
            if swapped {
                change(a, b, n, &mut s2, &mut s1);
            } else {
                change(a, b, n, &mut s1, &mut s2);
            }
        } else {
            swapped = !swapped;
        }
    }
    let res = if swapped {
        format!("{}{}", s2.iter().join(""), s1.iter().join(""))
    } else {
        format!("{}{}", s1.iter().join(""), s2.iter().join(""))
    };
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
