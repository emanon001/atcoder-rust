#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::*;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::collections::*;

fn solve() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let order = av
        .into_iter()
        .enumerate()
        .sorted_by_key(|(_, a)| *a)
        .collect::<Vec<_>>();
    let mut set = (0..n).into_iter().collect::<BTreeSet<_>>();
    let mut res = vec![-1; n];
    for (i, _) in order {
        let j = set.range(..i).next_back();
        let j = if let Some(&j) = j { j as isize + 1 } else { -1 };
        res[i] = j;
        set.remove(&i);
    }
    println!("{}", res.iter().join(" "));
}

fn solve_stack() {
    input! {
        n: usize,
        av: [i64; n]
    };

    let mut stack: Vec<(usize, i64)> = Vec::new();
    let mut res = vec![-1_isize; n];
    for (i, a) in av.into_iter().enumerate() {
        while let Some((_, b)) = stack.last() {
            if b > &a {
                break;
            }
            stack.pop();
        }
        res[i] = stack.last().map(|(j, _)| *j as isize + 1).unwrap_or(-1);
        stack.push((i, a));
    }
    println!("{}", res.iter().join(" "));
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(256 * 1024 * 1024)
        .spawn(|| {
            solve_stack();
        })
        .unwrap()
        .join()
        .unwrap();
}
