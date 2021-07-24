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
        sv: [String; 4]
    };

    let mut set = HashSet::new();
    set.insert("H".to_string());
    set.insert("2B".to_string());
    set.insert("3B".to_string());
    set.insert("HR".to_string());
    for s in sv {
        set.remove(&s);
    }
    let res = if set.is_empty() { "Yes" } else { "No" };
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
