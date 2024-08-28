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
        N: usize, X: i64, Y: i64,
        mut A: [i64; N],
        mut B: [i64; N],
    };

    A.sort();
    A.reverse();
    B.sort();
    B.reverse();

    let mut ans = N;

    let mut sum = 0;
    for (i, a) in A.into_iter().enumerate() {
        sum += a;
        if sum > X {
            chmin!(ans, i + 1);
            break;
        }
    }
    let mut sum = 0;
    for (i, b) in B.into_iter().enumerate() {
        sum += b;
        if sum > Y {
            chmin!(ans, i + 1);
            break;
        }
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
