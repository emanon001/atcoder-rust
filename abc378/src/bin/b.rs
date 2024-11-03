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
        QR: [(usize, usize); N],
        Q: usize,
        TD: [(Usize1, usize); Q],
    };

    for (t, d) in TD {
        let (q, r) = QR[t];
        let n = d / q;
        let m = d % q;
        let ans = if m <= r { q * n + r } else { q * (n + 1) + r };
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
