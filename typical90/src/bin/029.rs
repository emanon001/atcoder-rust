#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn compress_zahyo<T: Ord + std::hash::Hash>(
    zahyo: &[T],
) -> std::collections::HashMap<&T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x);
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x, i);
    }
    map
}

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
        _: usize, n: usize,
        lrv: [(usize, usize); n]
    };

    let mut zahyo = Vec::new();
    for &(l, r) in &lrv {
        zahyo.push(l);
        zahyo.push(r);
    }
    let zahyo = compress_zahyo(&zahyo);
    let mut height = vec![0; zahyo.len()];
    for (l, r) in &lrv {
        let l = zahyo[l];
        let r = zahyo[r];
        let mut h = 0;
        for x in l..=r {
            chmax!(h, height[x]);
        }
        h += 1;
        for x in l..=r {
            height[x] = h;
        }
        println!("{}", h);
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
