#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

pub fn compress_zahyo<T: Ord + std::hash::Hash>(
    zahyo: &[T],
) -> std::collections::HashMap<&T, usize> {
    let mut set = std::collections::BTreeSet::new();
    for x in zahyo {
        set.insert(x);
    }
    let mut map = std::collections::HashMap::new();
    for (i, x) in set.into_iter().enumerate() {
        map.insert(x, i);
    }
    map
}

#[derive(Debug)]
enum Event {
    Begin(usize),
    End(usize),
}

fn solve() {
    input! {
        n: usize,
        abv: [(usize, usize); n]
    };

    let mut events = Vec::new();
    for (a, b) in abv {
        events.push(Event::Begin(a));
        events.push(Event::End(a + b));
    }
    events.sort_by_key(|e| match e {
        &Event::Begin(x) => x,
        &Event::End(x) => x,
    });

    let mut res = vec![0_usize; n + 1];
    let mut cur = 0_usize;
    let mut k = 0_usize;
    for e in events {
        match e {
            Event::Begin(x) => {
                res[k] += x - cur;
                cur = x;
                k += 1;
            }
            Event::End(x) => {
                res[k] += x - cur;
                cur = x;
                k -= 1;
            }
        }
    }
    println!("{}", res[1..].into_iter().join(" "));
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
