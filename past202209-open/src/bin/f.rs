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
        A: [isize; N],
        X: [[isize]; N],
        Q: usize,
        Y: [[isize]; Q]
    };

    let mut a_set = vec![HashSet::new(); N];
    for i in 0..N {
        for &x in &X[i] {
            a_set[i].insert(x);
        }
    }

    for yv in Y {
        let mut ans = (-1_isize, -1_isize);
        for (i, &a) in A.iter().enumerate() {
            let ok = yv.iter().all(|y| !a_set[i].contains(y));
            if ok {
                chmax!(ans, (a, i as isize + 1));
            }
        }
        println!("{}", ans.1);
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
