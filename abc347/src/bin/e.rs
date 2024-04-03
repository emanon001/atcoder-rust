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
        N: usize, Q: usize,
        X: [Usize1; Q],
    };

    let mut set = HashSet::new();
    let mut cusum = vec![0; Q + 1];
    let mut event_indexes = vec![vec![]; N];
    for (i, x) in X.into_iter().enumerate() {
        event_indexes[x].push(i);
        if set.contains(&x) {
            set.remove(&x);
        } else {
            set.insert(x);
        }
        cusum[i + 1] = cusum[i] + set.len();
    }

    let mut ans = vec![0; N];
    for i in 0..N {
        let mut s = None;
        for &j in &event_indexes[i] {
            if s.is_none() {
                s = Some(j);
            } else {
                let e = j;
                ans[i] += cusum[e] - cusum[s.unwrap()];
                s = None;
            }
        }
        if let Some(s) = s {
            ans[i] += cusum[Q] - cusum[s];
        }
    }
    println!("{}", ans.into_iter().join(" "));
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
