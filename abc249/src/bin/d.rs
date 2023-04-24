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
        n: usize,
        av: [usize; n]
    };

    let mut counts: HashMap<usize, u64> = HashMap::new();
    let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for &a in &av {
        let mut m = a;
        *counts.entry(a).or_insert(0) += 1;
        while m <= 2 * 10.pow(5) {
            if map.contains_key(&m) && map[&m].contains(&a) {
                break;
            }
            map.entry(m).or_insert(HashSet::new()).insert(a);
            m += a;
        }
    }
    let mut res = 0_u64;
    for (a, a_count) in &counts {
        let set = &map[a];
        for b in set {
            let b_count = &counts[b];
            let c = a / b;
            if !counts.contains_key(&c) {
                continue;
            }
            let c_count = &counts[&c];
            res += a_count * b_count * c_count;
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
