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
        n: usize,
        av: [i32; n],
        d: usize,
        lrv: [(usize, usize); d]
    };

    let mut cumax1 = vec![0; n + 1];
    for i in 0..n {
        chmax!(cumax1[i + 1], cumax1[i]);
        chmax!(cumax1[i + 1], av[i]);
    }
    let mut cumax2 = vec![0; n + 1];
    for i in 0..n {
        chmax!(cumax2[i + 1], cumax2[i]);
        chmax!(cumax2[i + 1], av[n - i - 1]);
    }

    for (l, r) in lrv {
        let res = (cumax1[l - 1]).max(cumax2[n - r]);
        println!("{}", res);
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
