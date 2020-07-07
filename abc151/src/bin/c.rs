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
        n: usize, m: usize,
        psv: [(Usize1, String); m]
    };

    let mut ac_count = 0;
    let mut wa_count = 0;
    let mut solved = vec![false; n];
    let mut wa_counts = vec![0; n];
    for (p, s) in psv {
        let s = &s[..];
        match s {
            "AC" => {
                if !solved[p] {
                    ac_count += 1;
                    wa_count += wa_counts[p];
                    solved[p] = true;
                }
            }
            "WA" => {
                if !solved[p] {
                    wa_counts[p] += 1;
                }
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", ac_count, wa_count);
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
