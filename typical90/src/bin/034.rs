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

    let mut map = HashMap::new();
    let mut ans = 0;
    let mut l = 0;
    let mut r = 0;
    while r < N {
        *map.entry(A[r]).or_insert(0) += 1;
        while map.len() > K && l < r {
            let c = map.get_mut(&A[l]).expect("A[l] is some value");
            *c -= 1;
            if *c == 0 {
                map.remove(&A[l]);
            }
            l += 1;
        }
        r += 1;
        chmax!(ans, r - l);
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
