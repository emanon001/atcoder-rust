#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
use std::cmp::Ordering;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize, Q: usize,
        LRX: [(Usize1, Usize1, i64); Q],
    };

    let mut imos = vec![0; N];
    for (l, r, x) in LRX {
        imos[l] += x;
        if r + 1 < N {
            imos[r + 1] -= x;
        }
    }
    for i in 0..N - 1 {
        imos[i + 1] += imos[i];
    }
    let ans = imos
        .into_iter()
        .tuple_windows()
        .map(|(a, b)| match a.cmp(&b) {
            Ordering::Greater => ">",
            Ordering::Equal => "=",
            Ordering::Less => "<",
        })
        .join("");
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
