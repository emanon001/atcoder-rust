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
        H: usize, W: usize,
        S: [Chars; H],
    };

    let mut top = H;
    let mut bottom = 0;
    let mut left = W;
    let mut right = 0;
    for i in 0..H {
        for j in 0..W {
            if S[i][j] == '#' {
                chmin!(top, i);
                chmax!(bottom, i);
                chmin!(left, j);
                chmax!(right, j);
            }
        }
    }
    let mut is_ok = true;
    for i in top..=bottom {
        for j in left..=right {
            if S[i][j] == '.' {
                is_ok = false;
            }
        }
    }
    let ans = if is_ok { "Yes" } else { "No" };
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
