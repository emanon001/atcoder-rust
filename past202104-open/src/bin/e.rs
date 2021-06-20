#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn pop_front(a: &mut VecDeque<usize>, pos: usize) -> Option<usize> {
    if a.len() < pos {
        return None;
    }
    let mut backup = Vec::new();
    for _ in 0..pos - 1 {
        let x = a.pop_front().unwrap();
        backup.push(x);
    }
    let res = a.pop_front();
    for x in backup.into_iter().rev() {
        a.push_front(x);
    }
    res
}

fn pop_back(a: &mut VecDeque<usize>, pos: usize) -> Option<usize> {
    if a.len() < pos {
        return None;
    }
    let mut backup = Vec::new();
    for _ in 0..pos - 1 {
        let x = a.pop_back().unwrap();
        backup.push(x);
    }
    let res = a.pop_back();
    for x in backup.into_iter().rev() {
        a.push_back(x);
    }
    res
}

fn solve() {
    input! {
        n: usize,
        s: Chars
    };

    let mut a = VecDeque::new();
    for i in 0..n {
        let ch = s[i];
        let i = i + 1;
        match ch {
            'L' => {
                a.push_front(i);
            }
            'R' => {
                a.push_back(i);
            }
            'A' => {
                let res = if let Some(res) = pop_front(&mut a, 1) {
                    res.to_string()
                } else {
                    "ERROR".to_string()
                };
                println!("{}", res);
            }
            'B' => {
                let res = if let Some(res) = pop_front(&mut a, 2) {
                    res.to_string()
                } else {
                    "ERROR".to_string()
                };
                println!("{}", res);
            }
            'C' => {
                let res = if let Some(res) = pop_front(&mut a, 3) {
                    res.to_string()
                } else {
                    "ERROR".to_string()
                };
                println!("{}", res);
            }
            'D' => {
                let res = if let Some(res) = pop_back(&mut a, 1) {
                    res.to_string()
                } else {
                    "ERROR".to_string()
                };
                println!("{}", res);
            }
            'E' => {
                let res = if let Some(res) = pop_back(&mut a, 2) {
                    res.to_string()
                } else {
                    "ERROR".to_string()
                };
                println!("{}", res);
            }
            'F' => {
                let res = if let Some(res) = pop_back(&mut a, 3) {
                    res.to_string()
                } else {
                    "ERROR".to_string()
                };
                println!("{}", res);
            }
            _ => unreachable!()
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
