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
        s: Chars, k: usize
    };

    let mut res = BTreeSet::new();
    for perm in (0..s.len()).permutations(s.len()) {
        let mut v = Vec::new();
        for i in perm {
            v.push(s[i]);
        }
        res.insert(v.iter().join(""));
    }
    let mut i = 0;
    for s in res {
        if i == k - 1 {
            println!("{}", s);
            return;
        }
        i += 1;
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
