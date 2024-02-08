#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn compress_zahyo<T: Ord + std::hash::Hash + Clone>(
    zahyo: &[T],
) -> (
    std::collections::HashMap<T, usize>,
    std::collections::HashMap<usize, T>,
) {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x.clone());
    }
    let mut map = std::collections::HashMap::new();
    let mut inverse_map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x.clone(), i);
        inverse_map.insert(i, x);
    }
    (map, inverse_map)
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [i64; N],
    };

    let mut counts = HashMap::new();
    for &a in &A {
        *counts.entry(a).or_insert(0_isize) += 1;
    }
    let (zahyo, inverse_zahyo) = compress_zahyo(&A);
    let mut three_count = 0;
    for i in 0..zahyo.len() {
        match (
            inverse_zahyo.get(&i),
            inverse_zahyo.get(&(i + 1)),
            inverse_zahyo.get(&(i + 2)),
        ) {
            (Some(&a), Some(&b), Some(&c)) => {
                if a + 1 == b && b + 1 == c {
                    loop {
                        if counts[&a] > 0 && counts[&b] > 0 && counts[&c] > 0 {
                            three_count += 1;
                            counts.insert(a, counts[&a] - 1);
                            counts.insert(b, counts[&b] - 1);
                            counts.insert(c, counts[&c] - 1);
                        } else {
                            break;
                        }
                    }
                }
            }
            _ => break,
        }
    }
    let ans = N - three_count * 3;
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
