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
        sv: [String; n]
    };

    let mut table = HashMap::new();
    for s in sv {
        *table.entry(s).or_insert(0) += 1;
    }
    let keys = vec![
        "AC".to_string(),
        "WA".to_string(),
        "TLE".to_string(),
        "RE".to_string(),
    ];
    for k in keys {
        let v = table.get(&k).unwrap_or(&0);
        println!("{} x {}", k, v);
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
