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
        P: [Usize1; N],
        Q: [Usize1; N],
    };

    let q_to_i = Q
        .iter()
        .enumerate()
        .map(|(i, q)| (*q, i))
        .collect::<HashMap<_, _>>();
    let ans = (0..N)
        .map(|q| {
            let i = q_to_i[&q];
            Q[P[i]] + 1
        })
        .join(" ");
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
