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
        n: usize,
        mut lrv: [(usize, usize); n]
    };

    lrv.sort_by_key(|(l, _)| *l);
    lrv.push((10.pow(6), 10.pow(6)));
    let mut res = Vec::new();
    let mut cur = lrv[0];
    for (l, r) in lrv.into_iter().skip(1) {
        if l <= cur.1 {
            chmax!(cur.1, r);
        } else {
            res.push(cur);
            cur = (l, r);
        }
    }
    for (l, r) in res {
        println!("{} {}", l, r);
    }
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
