#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[macro_export]
macro_rules! chmin {
    ($ min : expr , $ v : expr ) => {{
        let v = $v;
        if $min > v {
            $min = v;
            true
        } else {
            false
        }
    }};
}

fn solve() {
    input! {
        n: usize,
        p: [usize; n]
    };

    for i in (0..n - 1).rev() {
        let mut set = (1..=n).collect::<BTreeSet<_>>();
        let mut new_perm = Vec::new();
        for j in 0..i {
            new_perm.push(p[j]);
            set.remove(&p[j]);
        }
        let a = p[i];
        if let Some(&b) = set.range(..a).next_back() {
            new_perm.push(b);
            set.remove(&b);
            for c in set.into_iter().rev() {
                new_perm.push(c);
            }
            println!("{}", new_perm.iter().join(" "));
            return;
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
