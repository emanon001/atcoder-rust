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
        n: usize, k: usize,
        sv: [Chars; n]
    };

    let mut res = 0;
    for bits in 0..(1 << n) {
        let mut map = HashMap::new();
        for i in 0..n {
            if (bits >> i) & 1 == 0 {
                continue;
            }
            for &ch in &sv[i] {
                *map.entry(ch).or_insert(0_usize) += 1;
            }
        }
        let c = map.into_iter().filter(|(_, v)| v == &k).count();
        chmax!(res, c);
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
