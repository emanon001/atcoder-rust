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
        S: Chars,
    };
    let mut i = 0;
    let mut ans = Vec::new();
    while i < N {
        match S[i] {
            '/' => {
                if i + 1 < N && S[i + 1] == '*' {
                    // start comment
                    let mut end_found = false;
                    for j in i + 2..N - 1 {
                        if S[j] == '*' && S[j + 1] == '/' {
                            // end comment
                            end_found = true;
                            i = j + 2;
                            break;
                        }
                    }
                    if !end_found {
                        for j in i..N {
                            ans.push(S[j]);
                        }
                        i = N;
                    }
                } else {
                    ans.push('/');
                    i += 1;
                }
            }
            ch => {
                ans.push(ch);
                i += 1;
            }
        };
    }
    println!("{}", ans.iter().join(""));
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
