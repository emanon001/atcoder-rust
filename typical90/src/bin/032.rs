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
    ($ min : expr , $ v : expr ) => {
        if $min > $v {
            $min = $v;
            true
        } else {
            false
        }
    };
}

fn solve() {
    input! {
        n: usize,
        time_table: [[i64; n]; n],
        m: usize,
        xyv: [(Usize1, Usize1); m]
    };

    let ng_set = xyv.into_iter().collect::<HashSet<_>>();
    let inf = 1_i64 << 60;
    let mut res = inf;
    'outer: for perm in (0..n).into_iter().permutations(n) {
        for (&x, &y) in perm.iter().tuple_windows() {
            if ng_set.contains(&(x.min(y), x.max(y))) {
                continue 'outer;
            }
        }
        let mut sum = 0;
        for i in 0..n {
            sum += time_table[perm[i]][i];
        }
        chmin!(res, sum);
    }
    let res = if res == inf { -1 } else { res };
    println!("{}", res);
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
