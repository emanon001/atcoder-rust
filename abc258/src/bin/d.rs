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
        n: usize, x: usize,
        abv: [(u64, u64); n]
    };

    let mut res = 1_u64 << 60;
    let mut base_cost = 0_u64;
    let mut set = BTreeSet::new();
    for i in 0..n {
        if x < i + 1 {
            break;
        }
        let (a, b) = abv[i];
        set.insert(b);
        base_cost += a + b;
        let rest_count = (x - (i + 1)) as u64;
        let cost = base_cost + set.iter().next().unwrap() * rest_count;
        chmin!(res, cost);
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
