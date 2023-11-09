#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
#[allow(unused_imports)]
use proconio::marker::*;
use proconio::{input, source::line::LineSource};
#[allow(unused_imports)]
use std::collections::*;
use std::io::{stdin, BufReader};

fn solve() {
    let mut source = LineSource::new(BufReader::new(stdin()));
    macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut source, $($tt)*)));
    input! {
        n: usize,
        q: usize,
    };

    let mut box_numbers = vec![BTreeSet::new(); n + 10];
    let mut number_boxes = vec![BTreeSet::new(); 2 * 10.pow(5) + 10];
    for q_i in 0..q {
        input! {
            kind: usize
        };
        match kind {
            1 => {
                input! {
                    i: usize, j: usize,
                };
                box_numbers[j].insert((i, q_i));
                number_boxes[i].insert(j);
            }
            2 => {
                input! {
                    i: usize
                };
                println!("{}", box_numbers[i].iter().map(|(n, _)| n).join(" "));
            }
            3 => {
                input! {
                    i: usize
                };
                println!("{}", number_boxes[i].iter().join(" "));
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
