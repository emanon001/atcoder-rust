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

fn solve() {
    input_interactive! {
        n: usize,
        fs: [(i64, i64); n]
    };

    let mut taste_map = HashMap::new();
    for (i, (f, s)) in fs.into_iter().enumerate() {
        taste_map.entry(f).or_insert(BTreeSet::new()).insert((s, i));
    }
    let mut ans = 0_i64;
    // 異なる味
    let top2 = taste_map
        .values()
        .map(|set| set.last().unwrap().0)
        .sorted()
        .rev()
        .take(2)
        .collect_vec();
    if top2.len() >= 2 {
        chmax!(ans, top2[0] + top2[1]);
    }
    // 同じ味
    let top = taste_map
        .into_values()
        .map(|set| {
            if set.len() >= 2 {
                let mut iter = set.iter();
                let last = iter.next_back().unwrap();
                let last2 = iter.next_back().unwrap();
                last.0 + last2.0 / 2
            } else {
                0
            }
        })
        .max()
        .unwrap();
    chmax!(ans, top);
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
