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
        N: usize, M: usize,
        ABC: [(Usize1, Usize1, Usize1); M],
    };

    let mut ans = 0;
    for bits in 0..1 << N {
        let mut set = HashSet::new();
        let mut ok = true;
        for &(a, b, c) in &ABC {
            match (bits >> a & 1 == 1, bits >> b & 1 == 1, bits >> c & 1 == 1) {
                (true, true, true) => {
                    ok = false;
                    break;
                }
                (true, true, false) => {
                    set.insert(c);
                }
                (true, false, true) => {
                    set.insert(b);
                }
                (false, true, true) => {
                    set.insert(a);
                }
                _ => {}
            }
        }
        if ok {
            chmax!(ans, set.len());
        }
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
