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
        A: [usize; N]
    };

    let mut position_map = BTreeMap::new();
    for (i, &a) in A.iter().enumerate() {
        position_map.entry(a).or_insert(vec![]).push(i);
    }
    let mut ans = vec![0; N];
    let mut x = 1;
    for (_, positions) in position_map {
        for i in positions {
            ans[i] = x;
            x += 1;
        }
    }
    println!("{}", ans.iter().join(" "));
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
