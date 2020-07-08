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
        n: usize
    };

    let mut ht_counts = HashMap::new();
    for x in 1..=n {
        let chars = x
            .to_string()
            .chars()
            .map(|ch| ch.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
        let head = chars[0];
        let tail = chars[chars.len() - 1];
        *ht_counts.entry((head, tail)).or_insert(0) += 1;
    }
    let mut res = 0_usize;
    for h in 1..=9 {
        for t in 1..=9 {
            match (ht_counts.get(&(h, t)), ht_counts.get(&(t, h))) {
                (Some(&a), Some(&b)) => {
                    res += a * b;
                }
                _ => {}
            }
        }
    }
    println!("{}", res);
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
