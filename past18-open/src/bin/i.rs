#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input_interactive;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug)]
struct Item {
    v: char,
    prev: Option<usize>,
    next: Option<usize>,
}

#[allow(non_snake_case)]
fn solve() {
    input_interactive! {
        Q: usize,
    };

    let mut list: Vec<Vec<Item>> = vec![vec![Item {
        v: ' ',
        prev: None,
        next: None,
    }]];
    let mut i = 0;
    let mut prev_j = 0;
    let mut cursor_w = 1;
    for _ in 0..Q {
        input_interactive! {
            k: usize,
        };
        match k {
            1 => {
                input_interactive! {
                    c: char,
                };
                list[i].push(Item {
                    v: c,
                    prev: None,
                    next: None,
                });
                let new_prev_j = list[i].len() - 1;
                if let Some(next_j) = list[i][prev_j].next {
                    list[i][new_prev_j].next = Some(next_j);
                    list[i][next_j].prev = Some(new_prev_j);
                }
                list[i][prev_j].next = Some(new_prev_j);
                list[i][new_prev_j].prev = Some(prev_j);
                prev_j = new_prev_j;
                cursor_w += 1;
            }
            2 => {
                prev_j = 0;
                cursor_w = 1;
            }
            3 => {
                i += 1;
                prev_j = 0;
                cursor_w = 1;
                list.push(vec![Item {
                    v: ' ',
                    prev: None,
                    next: None,
                }]);
            }
            _ => unreachable!(),
        }
    }
    println!("{} {}", i + 1, cursor_w);
    for v in list {
        let mut ans = vec!['#'];
        let mut j = Some(0);
        while let Some(jj) = j {
            ans.push(v[jj].v);
            j = v[jj].next;
        }
        println!("{}", ans.iter().join(""));
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
