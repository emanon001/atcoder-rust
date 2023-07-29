#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        m: usize,
        kv: [[Usize1]; m]
    };

    let k_set = kv
        .into_iter()
        .map(|v| v.into_iter().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    for i in 0..n {
        for j in i + 1..n {
            let is_ok = k_set.iter().any(|set| set.contains(&i) && set.contains(&j));
            if !is_ok {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
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
