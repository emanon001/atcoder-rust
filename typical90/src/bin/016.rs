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
        N: i64,
        A: i64, B: i64, C: i64,
    };

    let mut ans = i64::max_value();
    for a in 0..=9999 {
        for b in 0..=9999 {
            let ab_sum = a * A + b * B;
            if ab_sum > N {
                break;
            }
            if (N - ab_sum) % C != 0 {
                continue;
            }
            let c = (N - ab_sum) / C;
            chmin!(ans, a + b + c);
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
