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
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize, m: usize,
        av: [usize; n]
    };

    let mut res = n;
    let mut counts = vec![0; n];
    let mut set = (0..=n).collect::<BTreeSet<_>>();
    for i in 0..m {
        let a = av[i];
        counts[a] += 1;
        set.remove(&a);
    }
    chmin!(res, *set.iter().next().unwrap());
    for i in m..n {
        let a = av[i];
        counts[a] += 1;
        set.remove(&a);
        let b = av[i - m];
        counts[b] -= 1;
        if counts[b] == 0 {
            set.insert(b);
        }
        chmin!(res, *set.iter().next().unwrap());
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
