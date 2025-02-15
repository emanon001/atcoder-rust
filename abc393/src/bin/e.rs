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
        N: usize, K: usize,
        A: [usize; N],
    };

    let max_a = 10.pow(6);
    let mut a_counts = vec![0; max_a + 1];
    for &a in &A {
        a_counts[a] += 1;
    }

    let mut multiple_counts = vec![0; max_a + 1];
    for d in 1..=max_a {
        let mut c = 0;
        for m in (d..=max_a).step_by(d) {
            c += a_counts[m];
        }
        multiple_counts[d] = c;
    }

    let mut ans = vec![1; max_a + 1];
    for d in 1..=max_a {
        if multiple_counts[d] >= K {
            for m in (d..=max_a).step_by(d) {
                chmax!(ans[m], d);
            }
        }
    }

    let ans = A.into_iter().map(|a| ans[a]).join("\n");
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
