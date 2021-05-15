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
        s: Chars
    };

    let mut set1 = HashSet::new();
    let mut set2 = HashSet::new();
    for i in 0..10 {
        if s[i] == 'o' {
            set1.insert(i);
        } else if s[i] == 'x' {
            set2.insert(i);
        }
    }
    let mut res = 0_i64;
    for a in 0..10 {
        for b in 0..10 {
            for c in 0..10 {
                for d in 0..10 {
                    let mut is_ok = true;
                    let v = vec![a, b, c, d];
                    for x in &set1 {
                        if !v.contains(&x) {
                            is_ok = false;
                        }
                    }
                    for x in &set2 {
                        if v.contains(&x) {
                            is_ok = false;
                        }
                    }
                    if is_ok {
                        res += 1;
                    }
                }
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
