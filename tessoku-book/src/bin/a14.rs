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
        n: usize, k: i64,
        av: [i64; n],
        bv: [i64; n],
        cv: [i64; n],
        dv: [i64; n],
    };

    let mut ab_set = HashSet::new();
    for &a in &av {
        for &b in &bv {
            let ab = a + b;
            ab_set.insert(ab);
        }
    }
    let mut cd_set = HashSet::new();
    for &c in &cv {
        for &d in &dv {
            let cd = c + d;
            cd_set.insert(cd);
        }
    }
    for &ab in &ab_set {
        let rest = k - ab;
        if cd_set.contains(&rest) {
            println!("Yes");
            return;
        }
    }
    println!("No");
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
