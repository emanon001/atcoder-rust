#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: Chars
    };

    let len = n.len();
    let mut res = 0_u64;
    for nv in n.into_iter().permutations(len) {
        for i in 1..len {
            let s1 = nv[..i].iter().collect::<String>();
            let s2 = nv[i..].iter().collect::<String>();
            if s1.starts_with("0") || s2.starts_with("0") {
                continue;
            }
            let a: u64 = s1.parse().unwrap();
            let b: u64 = s2.parse().unwrap();
            let c = a * b;
            chmax!(res, c);
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
