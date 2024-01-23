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
        S: Chars
    };

    let mut count_map = HashMap::new();
    for i in 0..S.len() - 1 {
        let key = format!("{}{}", S[i], S[i + 1]);
        *count_map.entry(key).or_insert(0) += 1;
    }
    let mut tree = BTreeSet::new();
    for (key, count) in count_map {
        tree.insert((std::cmp::Reverse(count), key));
    }
    let ans = &tree.iter().next().unwrap().1;
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
