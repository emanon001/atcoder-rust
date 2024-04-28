#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmax {
    ($ max : expr , $ v : expr ) => {{
        let v = $v;
        if $max < v {
            $max = v;
            true
        } else {
            false
        }
    }};
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        L: u128, R: u128,
    };

    let mut ans = vec![];
    let mut l = L;
    while l < R {
        let mut r = l;
        for i in 0..=60 {
            let two_pow = 2.pow(i);
            if l == 0 || (two_pow <= l && l % two_pow == 0) {
                let j = l / two_pow;
                let tmp_r = two_pow * (j + 1);
                if tmp_r <= R {
                    chmax!(r, tmp_r);
                }
            }
        }
        ans.push((l, r));
        l = r;
    }
    println!("{}", ans.len());
    for (l, r) in ans {
        println!("{} {}", l, r);
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
