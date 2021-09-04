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
        s1: String,
        s2: String,
        s3: String
    };

    let mut set = HashSet::new();
    set.insert("ABC".to_string());
    set.insert("ARC".to_string());
    set.insert("AGC".to_string());
    set.insert("AHC".to_string());
    set.remove(&s1);
    set.remove(&s2);
    set.remove(&s3);
    for res in set {
        println!("{}", res);
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
