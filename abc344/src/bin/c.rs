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
        A: [i64; N],
        M: usize,
        B: [i64; M],
        L: usize,
        C: [i64; L],
        Q: usize,
        X: [i64; Q],
    };

    let mut set = HashSet::new();
    for i in 0..N {
        for j in 0..M {
            set.insert(A[i] + B[j]);
        }
    }
    for x in X {
        let is_ok = C.iter().any(|&c| set.contains(&(x - c)));
        let ans = if is_ok { "Yes" } else { "No" };
        println!("{}", ans);
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
