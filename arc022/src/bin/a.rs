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
        s: Chars
    };

    let s = s
        .into_iter()
        .map(|ch| ch.to_ascii_uppercase())
        .collect::<Vec<_>>();
    let mut i_ok = false;
    let mut c_ok = false;
    let mut t_ok = false;
    for ch in s {
        match ch {
            'I' => i_ok = true,
            'C' => {
                if i_ok {
                    c_ok = true
                }
            }
            'T' => {
                if c_ok {
                    t_ok = true
                }
            }
            _ => {}
        }
    }
    let res = if t_ok { "YES" } else { "NO" };
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
