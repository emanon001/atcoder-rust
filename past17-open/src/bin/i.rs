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
        N: usize,
        S: [String; N],
    };

    let mut count_map = HashMap::new();
    for s in &S {
        *count_map.entry(s).or_insert(0_u64) += 1;
    }
    let mut ans = 0_u64;
    for s in &S {
        let len = s.len();
        let chars = s.chars().collect::<Vec<_>>();
        let mut t_set = HashSet::new();
        for bits in 0..1 << len {
            let mut t = String::new();
            for i in 0..len {
                if (bits >> i) & 1 == 1 {
                    t.push(chars[i]);
                }
            }
            t_set.insert(t);
        }
        for t in t_set {
            if let Some(c) = count_map.get(&t) {
                ans += *c;
            }
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
