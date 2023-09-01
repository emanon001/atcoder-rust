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
        _: usize, q: usize,
        queries: [(usize, usize, usize); q]
    };

    let mut follow_map = HashMap::new();
    for (t, a, b) in queries {
        match t {
            1 => {
                follow_map.entry(a).or_insert(HashSet::new()).insert(b);
            }
            2 => {
                if let Some(set) = follow_map.get_mut(&a) {
                    set.remove(&b);
                }
            }
            3 => {
                let is_ok = match (follow_map.get(&a), follow_map.get(&b)) {
                    (Some(set1), Some(set2)) => set1.contains(&b) && set2.contains(&a),
                    _ => false,
                };
                let res = if is_ok { "Yes" } else { "No" };
                println!("{}", res);
            }
            _ => unreachable!(),
        }
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
