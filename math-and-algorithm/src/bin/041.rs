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
        T: usize,
        N: usize,
        LR: [(usize, usize); N],
    };

    let mut imos = vec![0; T + 1];
    for (l, r) in LR {
        imos[l] += 1;
        imos[r] -= 1;
    }
    for i in 0..T {
        imos[i + 1] += imos[i];
    }
    let ans = imos.into_iter().take(T).join("\n");
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
