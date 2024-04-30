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
        mut A: [usize; N],
    };

    let mut a_set = BTreeSet::new();
    let mut a_to_i = vec![0; N + 1];
    for (i, &a) in A.iter().enumerate() {
        a_set.insert(a);
        a_to_i[a] = i;
    }
    let mut ans = vec![];
    for i in 0..N - 1 {
        let a = A[i];
        let b = *a_set.iter().next().unwrap();
        if a == b {
            a_set.remove(&a);
            continue;
        }
        let j = a_to_i[b];
        A.swap(i, j);
        ans.push((i + 1, j + 1));
        a_set.remove(&b);
        a_to_i[a] = j;
        a_to_i[b] = i;
    }
    println!("{}", ans.len());
    for (i, j) in ans {
        println!("{} {}", i, j);
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
