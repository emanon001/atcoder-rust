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
        N: usize, Q: usize,
        HT: [(char, Usize1); Q],
    };

    let mut l = 0_usize;
    let mut r = 1_usize;
    let mut ans = 0_usize;
    for (h, t) in HT {
        match h {
            'L' => {
                let mut cost = 1_usize << 30;
                // l, t, r
                if (l..r).contains(&t) {
                    chmin!(cost, t - l);
                }
                // l, r, t
                if (l..=t).contains(&r) {
                    chmin!(cost, l + N - t);
                }
                // r, l, t
                if (r..=t).contains(&l) {
                    chmin!(cost, t - l);
                }
                // r, t, l
                if (r..=l).contains(&t) {
                    chmin!(cost, l - t);
                }
                // t, l, r
                if (t..r).contains(&l) {
                    chmin!(cost, l - t);
                }
                // t, r, l
                if (t..=l).contains(&r) {
                    chmin!(cost, N - l + t);
                }
                // eprintln!("cost: {}", cost);
                ans += cost;
                l = t;
            }
            'R' => {
                let mut cost = 1_usize << 30;
                // l, t, r
                if (l..r).contains(&t) {
                    chmin!(cost, r - t);
                }
                // l, r, t
                if (l..=t).contains(&r) {
                    chmin!(cost, t - r);
                }
                // r, l, t
                if (r..=t).contains(&l) {
                    chmin!(cost, r + N - t);
                }
                // r, t, l
                if (r..l).contains(&t) {
                    chmin!(cost, t - r);
                }
                // t, l, r
                if (t..=r).contains(&l) {
                    chmin!(cost, N - r + t);
                }
                // t, r, l
                if (t..l).contains(&r) {
                    chmin!(cost, r - t);
                }
                // eprintln!("cost: {}", cost);
                ans += cost;
                r = t;
            }
            _ => unreachable!(),
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
