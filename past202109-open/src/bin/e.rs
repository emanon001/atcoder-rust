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
        N: usize, K: usize,
        C: [usize; N],
        P: [i64; N],
    };

    let mut map = HashMap::new();
    for (c, p) in C.into_iter().zip(P.into_iter()) {
        let v = map.entry(c).or_insert(i64::max_value());
        if p < *v {
            *v = p;
        }
    }
    let ans = if map.len() < K {
        -1
    } else {
        map.values().sorted().take(K).sum::<i64>()
    };
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
