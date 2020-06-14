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
        mut av: [usize; n]
    };

    let max = 1_000_000;
    let mut table = HashMap::new();
    av.sort();
    let mut res = 0;
    let mut can_div = vec![false; max + 1];
    let mut divs = HashSet::new();
    for a in av {
        *table.entry(a).or_insert(0) += 1;
        if can_div[a] {
            continue;
        }
        divs.insert(a);
        res += 1;
        can_div[a] = true;
        let mut j = a + a;
        while j <= max {
            can_div[j] = true;
            j += a;
        }
    }
    for (a, c) in table {
        if divs.contains(&a) && c > 1 {
            res -= 1;
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
