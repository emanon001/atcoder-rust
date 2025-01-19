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
        Q: usize,
    };

    let mut snake_list = VecDeque::new();
    let mut l_sum = 0;
    let mut offset = 0;
    for _ in 0..Q {
        input_interactive! {
            kind: usize,
        };
        match kind {
            1 => {
                input_interactive! {
                    l: usize,
                };
                snake_list.push_back((l_sum, l));
                l_sum += l;
            }
            2 => {
                let (_, l) = snake_list.pop_front().unwrap();
                offset += l;
            }
            3 => {
                input_interactive! {
                    k: Usize1,
                };
                let (pos, _) = snake_list[k];
                let ans = pos - offset;
                println!("{}", ans);
            }
            _ => unreachable!(),
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
