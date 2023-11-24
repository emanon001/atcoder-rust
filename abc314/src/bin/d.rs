#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input_interactive! {
        n: usize,
        mut s1: Chars,
        q: usize,
        queries: [(usize, usize, char); q],
    };

    let mut op = 0;
    let mut s2 = HashMap::new();
    for (t, x, c) in queries {
        match t {
            1 => {
                s2.insert(x - 1, c);
            }
            2 => {
                op = 2;
                for (i, c) in s2.drain() {
                    s1[i] = c;
                }
            }
            3 => {
                op = 3;
                for (i, c) in s2.drain() {
                    s1[i] = c;
                }
            }
            _ => unreachable!(),
        }
    }
    let ans = (0..n)
        .map(|i| {
            if let Some(c) = s2.get(&i) {
                *c
            } else if op == 0 {
                s1[i]
            } else if op == 2 {
                s1[i].to_ascii_lowercase()
            } else if op == 3 {
                s1[i].to_ascii_uppercase()
            } else {
                unreachable!()
            }
        })
        .join("");
    println!("{ans}");
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
