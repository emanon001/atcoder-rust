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

fn solve() {
    input! {
        n: usize, m: usize, d: i64,
        av: [i64; n],
        bv: [i64; m],
    };

    let mut ans = -1_i64;
    let b_set = bv.into_iter().collect::<BTreeSet<_>>();
    for a in av {
        let range = (a - d)..=(a + d);
        if let Some(b) = b_set.range(range).next_back() {
            let sum = a + b;
            chmax!(ans, sum);
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
