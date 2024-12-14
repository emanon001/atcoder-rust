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
        N: usize,
        S: [[usize]; N],
    };

    let mut ans = 0_usize;
    for bits in 0..1 << N {
        let mut s_count = 0;
        let mut counts = HashMap::new();
        for i in 0..N {
            if (bits >> i) & 1 == 1 {
                s_count += 1;
                for &a in &S[i] {
                    *counts.entry(a).or_insert(0) += 1;
                }
            }
        }
        if s_count < 2 {
            continue;
        }
        let is_ok = counts.iter().all(|(k, c)| !(k.is_even() && c == &s_count));
        if is_ok {
            ans += 1;
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
