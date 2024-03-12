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
        T: String,
        N: usize,
        S: [[String]; N],
    };

    let mut dp = HashMap::new();
    dp.insert("".to_string(), 0_isize);
    for s in S {
        let mut new_dp = HashMap::new();
        for si in s {
            for (k, v) in dp.clone() {
                let e = new_dp.entry(k.clone()).or_insert(v);
                chmin!(*e, v);
                let new_k = format!("{}{}", k, si);
                if T.starts_with(&new_k) {
                    let new_v = v + 1;
                    let e = new_dp.entry(new_k).or_insert(new_v);
                    chmin!(*e, new_v);
                }
            }
        }
        dp = new_dp;
    }
    let ans = dp.get(&T).unwrap_or(&-1);
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
