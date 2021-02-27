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
        n: i64
    };

    let mut set = HashSet::new();
    for a in 2..= ((n as f64).sqrt() as i64) {
        let mut x = a * a;
        while x <= n {
            set.insert(x);
            x *= a;
        }
    }
    let res = n - set.len() as i64;
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
