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
        sv: [String; n]
    };

    let set = sv.into_iter().collect::<HashSet<_>>();
    let mut map = HashMap::new();
    for s in set {
        let s = if s.starts_with('!') {
            s[1..].to_string()
        } else {
            s
        };
        *map.entry(s).or_insert(0) += 1;
    }
    for (s, c) in map {
        if c >= 2 {
            println!("{}", s);
            return;
        }
    }
    println!("satisfiable");
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
