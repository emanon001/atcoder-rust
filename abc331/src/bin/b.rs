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

fn solve() {
    input_interactive! {
        N: usize, S: usize, M: usize, L: usize,
    };

    let mut ans = usize::max_value();
    for s_c in 0..=100 {
        for m_c in 0..=100 {
            for l_c in 0..=100 {
                let n = s_c * 6 + m_c * 8 + l_c * 12;
                if n < N {
                    continue;
                }
                chmin!(ans, s_c * S + m_c * M + l_c * L);
            }
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
