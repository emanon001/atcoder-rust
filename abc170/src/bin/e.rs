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
        n: usize, q: usize,
        abv: [(usize, Usize1); n],
        cdv: [(Usize1, Usize1); q]
    };

    let max_group = 2 * 10.pow(5);
    let mut groups = vec![0; n];
    let mut min_set = BTreeSet::new();
    let mut max_set = vec![BTreeSet::new(); max_group];
    for (i, &(a, b)) in abv.iter().enumerate() {
        groups[i] = b;
        max_set[b].insert((a, i));
    }
    for i in 0..max_group {
        if let Some(&max) = max_set[i].iter().last() {
            min_set.insert(max);
        }
    }
    for (c, d) in cdv {
        let cg = groups[c];
        let cv = (abv[c].0, c);
        // remove min
        let max_c = max_set[cg].iter().last().unwrap().clone();
        min_set.remove(&max_c);
        if let Some(max) = max_set[d].iter().last() {
            min_set.remove(max);
        }
        // move
        max_set[cg].remove(&cv);
        max_set[d].insert(cv);
        groups[c] = d;
        // insert min
        if let Some(&max) = max_set[cg].iter().last() {
            min_set.insert(max);
        }
        if let Some(&max) = max_set[d].iter().last() {
            min_set.insert(max);
        }
        let (res, _) = min_set.iter().next().unwrap();
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
