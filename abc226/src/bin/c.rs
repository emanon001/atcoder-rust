#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn dfs(i: usize, used: &mut HashSet<usize>, tv: &[i64], av: &[Vec<usize>]) -> i64 {
    if used.contains(&i) {
        return 0;
    }
    used.insert(i);
    if av[i].is_empty() {
        return tv[i];
    }
    let mut res = tv[i];
    for &a in &av[i] {
        if used.contains(&a) {
            continue;
        }
        res += dfs(a, used, tv, av);
    }
    res
}

fn solve() {
    input! {
        n: usize,
    };

    let mut tv = vec![0; n];
    let mut kv = vec![0; n];
    let mut av = vec![Vec::new(); n];
    for i in 0..n {
        input! { t: i64 };
        tv[i] = t;
        input! { k: i64 };
        kv[i] = k;
        input! { mut a: [Usize1; k] };
        a.sort();
        a.reverse();
        av[i] = a;
    }
    let mut used = HashSet::new();
    let res = dfs(n - 1, &mut used, &tv, &av);
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
