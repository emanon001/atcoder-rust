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
        Q: usize,
        queries: [(Usize1, Usize1); Q],
    };

    let mut id_to_pos = vec![0; N];
    for (i, &p) in P.iter().enumerate() {
        id_to_pos[p] = i;
    }
    for (a, b) in queries {
        let ans = if id_to_pos[a] < id_to_pos[b] { a } else { b } + 1;
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
