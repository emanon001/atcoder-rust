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
        N: usize, M: usize,
        S: [Chars; N],
    };

    let mut ans = 1 << 30;
    for bits in 0..1 << N {
        let mut set = HashSet::new();
        let mut count = 0;
        for i in 0..N {
            if (bits >> i) & 1 == 1 {
                count += 1;
                for j in 0..M {
                    if S[i][j] == 'o' {
                        set.insert(j);
                    }
                }
            }
        }
        if set.len() == M {
            ans = ans.min(count);
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
