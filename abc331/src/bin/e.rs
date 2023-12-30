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
        N: usize, M: usize, L: usize,
        A: [usize; N],
        B: [usize; M],
        CD: [(Usize1, Usize1); L],
    };

    let sorted_b = B
        .into_iter()
        .enumerate()
        .sorted_by_key(|(_, b)| *b)
        .rev()
        .collect_vec();
    let ng_set = CD.into_iter().collect::<HashSet<_>>();
    let mut ans = 0;
    for (i, a) in A.into_iter().enumerate() {
        for (j, b) in &sorted_b {
            if ng_set.contains(&(i, *j)) {
                continue;
            }
            chmax!(ans, a + b);
            break;
        }
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
