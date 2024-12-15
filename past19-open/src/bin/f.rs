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
        X: String, Y: String,
        ST: [(String, String); N],
    };

    let mut relations = HashMap::new();
    for (s, t) in ST {
        relations.entry(s).or_insert(Vec::new()).push(t);
    }
    let mut visited = HashSet::new();
    visited.insert(X.clone());
    let mut que = VecDeque::new();
    que.push_back(X.clone());
    while let Some(x) = que.pop_front() {
        for y in relations.get(&x).unwrap_or(&Vec::new()) {
            if visited.contains(y) {
                continue;
            }
            visited.insert(y.clone());
            if y == &Y {
                println!("Yes");
                return;
            }
            que.push_back(y.clone());
        }
    }
    println!("No");
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
