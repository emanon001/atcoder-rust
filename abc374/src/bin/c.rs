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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        K: [i64; N],
    };

    let mut ans = 1_i64 << 60;
    for bits in 0..1 << N {
        let mut sum_a = 0_i64;
        let mut sum_b = 0_i64;
        for i in 0..N {
            if (bits >> i) & 1 == 1 {
                sum_a += K[i];
            } else {
                sum_b += K[i];
            }
        }
        chmin!(ans, sum_a.max(sum_b));
    }
    println!("{}", ans);
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
