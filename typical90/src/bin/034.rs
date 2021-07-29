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
        av: [i64; n]
    };

    let mut l = 0;
    let mut map = HashMap::new();
    let mut res = 0;
    for r in 0..n {
        *map.entry(av[r]).or_insert(0) += 1;
        if map.len() <= k {
            chmax!(res, r - l + 1);
        } else {
            while map.len() > k {
                let v = av[l];
                *map.get_mut(&v).unwrap() -= 1;
                if map[&v] == 0 {
                    map.remove(&v);
                }
                l += 1;
            }
        }
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
