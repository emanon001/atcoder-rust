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
        H: [isize; N],
    };

    let mut dp = vec![1 << 30; N];
    dp[0] = 0;
    for i in 0..N {
        if i + 1 < N {
            chmin!(dp[i + 1], dp[i] + (H[i] - H[i + 1]).abs());
        }
        if i + 2 < N {
            chmin!(dp[i + 2], dp[i] + (H[i] - H[i + 2]).abs());
        }
    }
    println!("{}", dp[N - 1]);
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
