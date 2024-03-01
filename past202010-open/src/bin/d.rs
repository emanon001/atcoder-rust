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
        S: Chars,
    };

    let mut ans = (1 << 30, 1 << 30, 1 << 30);
    for x in 0..N {
        let mut s = S.clone();
        for _ in 0..x {
            for i in 0..N - 1 {
                if s[i + 1] == '#' {
                    s[i] = '#';
                }
            }
        }
        for y in 0..N {
            let mut s2 = s.clone();
            for _ in 0..y {
                for i in (1..N).rev() {
                    if s2[i - 1] == '#' {
                        s2[i] = '#';
                    }
                }
            }
            if !s2.contains(&'.') {
                chmin!(ans, (x + y, x, y));
            }
        }
    }
    println!("{} {}", ans.1, ans.2);
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
