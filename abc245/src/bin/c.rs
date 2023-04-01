#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize, k: i64,
        av: [i64; n],
        bv: [i64; n],
    };

    let mut xv = vec![av[0], bv[0]].into_iter().collect::<HashSet<_>>();
    for i in 1..n {
        let mut next_xv = HashSet::new();
        for x in xv {
            let a = av[i];
            let b = bv[i];
            if (x - a).abs() <= k {
                next_xv.insert(a);
            }
            if (x - b).abs() <= k {
                next_xv.insert(b);
            }
        }
        if next_xv.is_empty() {
            println!("No");
            return;
        }
        xv = next_xv;
    }
    println!("Yes");
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
