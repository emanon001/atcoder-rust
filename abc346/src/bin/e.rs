#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        H: usize, W: usize, M: usize,
        TAX: [(usize, usize, usize); M],
    };

    let mut h_set = HashSet::new();
    let mut v_set = HashSet::new();
    let mut counts = BTreeMap::new();
    for (t, a, x) in TAX.into_iter().rev() {
        if t == 1 {
            // 行(横)
            if h_set.contains(&a) {
                continue;
            }
            h_set.insert(a);
            let c = W - v_set.len();
            *counts.entry(x).or_insert(0) += c;
        } else {
            // 列(縦)
            if v_set.contains(&a) {
                continue;
            }
            v_set.insert(a);
            let c = H - h_set.len();
            *counts.entry(x).or_insert(0) += c;
        }
    }
    let rest_c = H * W - counts.values().sum::<usize>();
    *counts.entry(0).or_insert(0) += rest_c;
    let counts = counts.into_iter().filter(|(_, v)| v > &0).collect_vec();
    println!("{}", counts.len());
    for (k, v) in counts {
        println!("{} {}", k, v);
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
