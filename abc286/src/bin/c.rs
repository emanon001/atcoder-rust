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
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize, a: u64, b: u64,
        s: Chars
    };

    let mut res = 1_u64 << 60;
    for a_count in 0..n {
        let mut sum = a * a_count as u64;
        for i in 0..n / 2 {
            let pos = (i + a_count) % n;
            let pos2 = ((n - i - 1) + a_count) % n;
            sum += if s[pos] == s[pos2] { 0 } else { b };
        }
        chmin!(res, sum);
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
