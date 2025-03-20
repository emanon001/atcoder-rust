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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [usize; N],
    };

    let mut front_set = HashSet::new();
    let mut back_map = HashMap::new();
    for &a in &A {
        *back_map.entry(a).or_insert(0) += 1;
    }
    let mut ans = 0;
    for a in A {
        let c = back_map.get_mut(&a).unwrap();
        if *c == 1 {
            back_map.remove(&a);
        } else {
            *c -= 1;
        }
        front_set.insert(a);
        chmax!(ans, front_set.len() + back_map.len());
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
