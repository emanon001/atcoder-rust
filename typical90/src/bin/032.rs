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

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        N: usize,
        A: [[i64; N]; N],
        M: usize,
        XY: [(Usize1, Usize1); M],
    };

    let mut ng_set = HashSet::new();
    for (x, y) in XY {
        ng_set.insert((x, y));
        ng_set.insert((y, x));
    }
    let inf = 1_i64 << 60;
    let mut ans = inf;
    for perm in (0..N).permutations(N) {
        let mut cost = 0;
        let mut prev = N;
        for (j, i) in perm.into_iter().enumerate() {
            if ng_set.contains(&(prev, i)) {
                cost = inf;
                break;
            }
            cost += A[i][j];
            prev = i;
        }
        chmin!(ans, cost);
    }
    let ans = if ans == inf { -1 } else { ans };
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
