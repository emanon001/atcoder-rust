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
        N: usize, Q: usize,
        mut S: Chars,
        queries: [(Usize1, char); Q],
    };

    let mut count = 0;
    for i in 2..N {
        if S[i - 2] == 'A' && S[i - 1] == 'B' && S[i] == 'C' {
            count += 1;
        }
    }
    for (x, c) in queries {
        if S[x] == c {
            println!("{}", count);
            continue;
        }
        count -= if is_abc(x, &S) { 1 } else { 0 };
        S[x] = c;
        count += if is_abc(x, &S) { 1 } else { 0 };
        println!("{}", count);
    }
}

fn is_abc(i: usize, s: &[char]) -> bool {
    match s[i] {
        'A' => i + 2 < s.len() && s[i + 1] == 'B' && s[i + 2] == 'C',
        'B' => i >= 1 && i + 1 < s.len() && s[i - 1] == 'A' && s[i + 1] == 'C',
        'C' => i >= 2 && s[i - 2] == 'A' && s[i - 1] == 'B',
        _ => false,
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
