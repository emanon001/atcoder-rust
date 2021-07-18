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
    ($ max : expr , $ v : expr ) => {
        if $max < $v {
            $max = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize, k: usize,
        cv: [i64; n]
    };

    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(cv[i]).or_insert(0) += 1;
    }
    let mut res = map.len();
    for i in k..n {
        *map.entry(cv[i]).or_insert(0) += 1;
        let x = cv[i - k];
        if map.get(&x).is_some() {
            let count = *map.get(&x).unwrap();
            if count == 1 {
                map.remove(&x);
            } else {
                map.insert(x, count - 1);
            }
        }
        chmax!(res, map.len());
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
