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
        A: [Usize1; N],
        W: [i64; N],
    };

    let mut box_list = vec![vec![]; N];
    for (i, &a) in A.iter().enumerate() {
        box_list[a].push(W[i]);
    }
    let mut ans = 0;
    for b in box_list {
        ans += b
            .iter()
            .sorted_by_key(|w| std::cmp::Reverse(*w))
            .skip(1)
            .sum::<i64>();
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
